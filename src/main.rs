use colored::Colorize;
use dialoguer::Select;
use seq_io::fasta::Reader;

use ambi::string_match::{self, TextMatch};

#[derive(Clone)]
enum SampleSource{
    Direct(Vec<char>),
    Record(String)
}

#[derive(Clone)]
struct SampleSet{
    target: SampleSource,
    patterns: Vec<String> 
}

pub enum MatchingState{
    Performance,
    Debug
}

fn main() {

    let matching_state = MatchingState::Debug;

    Select::new().items(&["Performance", "Debug"]);

    let target_gen = Reader::from_path("./data/gen.fasta").unwrap().records().next().unwrap().unwrap().seq.iter().map(|x| *x as char).collect::<String>();
    let sample_sets = vec![
        // Poem
        SampleSet{
            target: SampleSource::Record("./data/text.fasta".into()),
            patterns: vec![
                "Besen".into(),
                "Wasserstroeme".into(),
                "Eimer".into(),
                "Bist schon lange Knecht gewesen:".into()
            ]
        },
        // Virus
        SampleSet{
            target: SampleSource::Record("./data/Virus.fasta".into()),
            patterns: vec![
                "GTATTA".into(),
                "TTTCGAAA".into(),
                "AAATTGACG".into()
            ]
        },
        // Gene
        SampleSet{
            target: SampleSource::Record("./data/BA000002.fna".into()),
            patterns: vec![
                "GAATTC".into(),
                "GGATCC".into(),
                "ATTTAAAT".into(),
                target_gen
            ]
        }
    ];

    let scenario_options = &["poem","virus","gene","custom"];
    while let Ok(Some(index)) = Select::new().items(scenario_options)
    .with_prompt("Choose a matching scenario. Press 'q' or 'Esc' to exit").interact_opt() {
        let set = if index < 3 {
            sample_sets[index].clone()
        } else {
            let target: String = dialoguer::Input::new().with_prompt("Enter a text to search in: [There is currently a bug with umlauts and other special characters]").interact_text().expect("You had to input a sequence...");
            let mut patterns = vec![];
            while let Ok(p) = dialoguer::Input::new().with_prompt("Enter a pattern or nothing to end the input sequence").allow_empty(true).interact_text() {
                let result: String = p;
                if !result.is_empty() {
                    patterns.push(result);
                } else {
                    break;
                }
            }
            SampleSet{
                target: SampleSource::Direct(
                    target.chars().collect()
                ),
                patterns
            }
        };
        println!("Choosen scenario: {}", scenario_options[index].to_string().red());

        let text: Vec<char> = match set.target {
            SampleSource::Direct(t) => t,
            SampleSource::Record(path) => {
                let mut sequence = vec![];
                if let Some(Ok(rec)) = Reader::from_path(path).ok().and_then(|r| r.into_records().next()) {
                    sequence = rec.seq.iter().map(|x| *x as char).collect();
                }
                sequence
            }
        };

        // Match set
        let prepare_matcher: Box<dyn Fn(Vec<char>, &[char]) -> Box<dyn Fn(&Vec<char>) -> Vec<usize>>> = match Select::new().items(&["Naive", "Rabin Karp", "Automaton", "Knuth Morris Pratt", "Boyer Moore", "Native"]).with_prompt("Choose a matching algorithmn").interact() {
            Ok(0) => {
                // Naive
                println!("{}","NAIVE".to_string().red());
                Box::new(|pattern, sigma| {
                    Box::new(move |text| {
                        string_match::naive_match(text, &pattern)
                    })
                })
            },
            Ok(1) => {
                // Rabin Karp
                Box::new(|pattern, sigma| {
                    let rk = string_match::RabinKarp::new(pattern.as_slice(), sigma);
                    Box::new(move |text| {
                        // string_match::rabin_karp(text, &pattern, 2, 101);
                        rk.match_text(text)
                    })
                })
            },
            Ok(2) => {
                // Automaton
                Box::new(|pattern, _sigma| {
                    let automaton = string_match::Automat::new(pattern.as_slice());
                    Box::new(move |text| automaton.match_text(text.as_slice()))
                })
            },
            Ok(3) => {
                // Knuth Morris Pratt
                Box::new(|pattern, _sigma| {
                    let knuth_morris_pratt = string_match::KnuthMorrisPratt::new(pattern.as_slice());
                    Box::new(move |text| knuth_morris_pratt.match_text(text.as_slice()))
                })
            },
            Ok(4) => {
                // Boyer Moore
                Box::new(|pattern, _sigma| {
                    let bm = string_match::BoyerMoore::new(&pattern);
                    Box::new(move |text| bm.match_text(text))
                })
            },
            _ => {
                Box::new(|pattern: Vec<char>, _sigma: &[char]| {
                    Box::new(move |text: &Vec<char>| {
                        let text = text.iter().collect::<String>();
                        let pattern = pattern.iter().collect::<String>();
                        let what = text.as_str().match_indices(pattern.as_str()).map(|(idx, _)| idx).collect::<Vec<usize>>();
                        what
                    })
                })
            }
        };

        for pattern in &set.patterns {
            let chars: Vec<char> = pattern.chars().collect();

            // Alphabet
            let mut sigma = text.iter().chain(chars.iter()).cloned().collect::<Vec<char>>();
            sigma.sort_unstable();
            sigma.dedup();

            // Prepare pattern
            println!("Pattern P {}", pattern.to_string().yellow());
            let now = std::time::Instant::now();
            let match_text = prepare_matcher(chars, sigma.as_slice());
            let elapsed = now.elapsed().as_micros();
            println!("Prepared for search of pattern P in {}μs", elapsed.to_string().yellow());
            // Match against text
            let now = std::time::Instant::now();
            let result = match_text(&text);
            let elapsed = now.elapsed().as_micros();
            println!("Found pattern P at indices [{}] in {}μs", format_char_vec(result.as_slice()), elapsed.to_string().yellow());
        }
    }
}

fn format_char_vec(input: &[usize]) -> String {
    let mut string: String = "".into();
    for payload in input.iter().skip(1) {
        string = format!("{}, {}", string, (*payload).to_string().green());
    };
    if let Some(p) = input.iter().next() {
        string = format!("{}{}", (*p).to_string().green(), string);
    }
    string
}
