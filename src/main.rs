use colored::Colorize;
use dialoguer::Select;
use seq_io::fasta::Reader;

use crate::string_match::TextMatch;

pub mod string_match;

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
                "Eimer".into()
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

    while let Ok(index) = Select::new().items(&["poem","virus","gene","custom"]).with_prompt("Choose a matching scenario:").interact() {
        let set = if index < 3 {
            sample_sets[index].clone()
        } else {
            let target: String = dialoguer::Input::new().with_prompt("Enter a text to search in: [There is currently a bug with umlauts and other special characters]").interact_text().expect("You had to input a sequence...");
            let mut patterns = vec![];
            while let Ok(p) = dialoguer::Input::new().with_prompt("Enter a pattern or nothing to end the input sequence:").allow_empty(true).interact_text() {
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
        let prepare_matcher: Box<dyn Fn(Vec<char>) -> Box<dyn Fn(&Vec<char>) -> Vec<usize>>> = match Select::new().items(&["Naive", "Rabin Karp", "Automaton", "Knuth Morris Pratt", "Boyer Moore"]).with_prompt("Chose a matching algorithmn:").interact() {
            Ok(0) => {
                // Naive
                println!("{}",format!("NAIVE").red());
                Box::new(|pattern| {
                    Box::new(move |text| {
                        string_match::naive_match(text, &pattern)
                    })
                })
            },
            Ok(1) => {
                // Rabin Karp
                Box::new(|pattern| {
                    Box::new(move |text| string_match::rabin_karp(text, &pattern, 2, 101))
                })
            },
            Ok(2) => {
                // Automaton
                Box::new(|pattern| {
                    let automaton = string_match::Automat::new(pattern.as_slice());
                    Box::new(move |text| automaton.match_text(text.as_slice()))
                })
            },
            Ok(3) => {
                // Knuth Morris Pratt
                Box::new(|pattern| {
                    let knuth_morris_pratt = string_match::KnuthMorrisPratt::new(pattern.as_slice());
                    Box::new(move |text| knuth_morris_pratt.match_text(text.as_slice()))
                })
            },
            Ok(4) => {
                // Boyer Moore
                Box::new(|pattern| {
                    let bm = string_match::BoyerMoore::new(&pattern);
                    Box::new(move |text| bm.match_text(text))
                })
            },
            _ => panic!("You didn't chose anything... How?")
        };

        for pattern in &set.patterns {
            let chars = pattern.chars().collect();
            // Prepare pattern
            let now = std::time::Instant::now();
            let match_text = prepare_matcher(chars);
            println!("Pattern P {}", pattern.to_string().yellow());
            println!("Prepared for search of pattern P in {}μs", now.elapsed().as_micros().to_string().yellow());
            // Match against text
            let now = std::time::Instant::now();
            let result = match_text(&text);
            println!("Found pattern P at indices [{}] in {}μs", format_char_vec(result.as_slice()), now.elapsed().as_micros().to_string().yellow());
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
