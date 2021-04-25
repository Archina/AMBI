use std::fs::File;

use seq_io::fasta::Reader;

pub mod string_match{
    use std::collections::{HashMap, VecDeque};

    fn str_match<T>(start: usize, text: &[T], pattern: &[T]) -> bool where T: Eq {
        text[start..start+pattern.len()] == pattern[0..pattern.len()]
    }

    pub fn naive_match<T,P>(text: T, pattern: P) -> Vec<usize> where T: AsRef<str>, P: AsRef<str> {
        let mut matches= vec![];
        let text = text.as_ref();
        let pattern = pattern.as_ref();
        let t_count = text.chars().count();
        let p_count = pattern.chars().count();
        if p_count > t_count { panic!("pattern is longer than text"); }
        for t_i in 0..t_count-p_count+1 {
            if str_match(
                t_i,
                &text.chars().collect::<Vec<char>>(),
                &pattern.chars().collect::<Vec<char>>()
            ) {
                matches.push(t_i);
            }
        }
        matches
    }

    #[test]
    fn test_naive_matches() {
        let results = naive_match("I am sock a sock", "sock");
        assert_eq!(vec![5,12], results);
    }

    #[test]
    #[should_panic]
    fn test_naive_crash() {
        naive_match("I am", "sock tube colour nope");
    }

    pub fn rabin_karp<T>(text: T, pattern: T, d_base: u32, q_prime: u32) -> Vec<usize> where T: AsRef<str> {
        let mut matches= vec![];

        let text = text.as_ref();
        let pattern = pattern.as_ref();

        // let n = text.chars().count();
        let m = pattern.chars().count();

        let h = d_base.pow(m as u32 - 1) % q_prime;
        let mut p = 0;
        let mut t_s = 0;

        let mut buffer = VecDeque::<char>::with_capacity(m+1);

        for (_i, (p_i, t_i)) in pattern.chars().zip(text.chars()).enumerate(){
            p = (d_base * p + p_i as u32) % q_prime;
            t_s = (d_base * t_s + t_i as u32) % q_prime;
            buffer.push_back(t_i);
        }

        // println!("0: {:?}\n{}", buffer, t_s);
        let mut iter = text.chars().skip(m);
        let mut i = 0;
        loop {
            if t_s == p {
                matches.push(i);
            }
            if let (Some(pop), Some(push)) = (buffer.pop_front(),iter.next()) {
                i += 1;
                // println!("was: {}", t_s);
                // println!("pop: {}",(pop as u32 * h) % q_prime);
                // println!("push: {}",(push as u32) % q_prime);
                let t_next = ((t_s as i32 - (pop as u32 * h) as i32) * d_base as i32 + (push as u32) as i32).rem_euclid(q_prime as i32);
                // println!("is: {}", t_next);
                t_s = t_next as u32;
                buffer.push_back(push);
                // println!("{}: {:?}\n{}", i, buffer, t_s);
            } else {
                break;
            }
        }
        matches
    }

    pub trait TextMatch{
        fn match_text(&self, text: &[char]) -> Vec<usize>;
    }

    pub struct Automat{
        accept: usize,
        transition_functions: HashMap<
            (usize, char), usize
        >,
    }

    impl Automat{
        pub fn new(pattern: &Vec<char>) -> Self{
            let mut alphabet = pattern.clone();
            alphabet.sort_unstable();
            alphabet.dedup();
            let mut transition_functions = HashMap::new();
            // println!("ALPHA {:?}", alphabet);
            for idx in 0..=pattern.len() {
                for a in &alphabet {
                    if idx < pattern.len() && pattern[idx] == *a {
                        transition_functions.insert(
                            (idx, *a),
                            idx+1
                        );
                    } else {
                        // println!("({}, {}): {}", idx, a, &pattern[0..idx].iter().collect::<String>());
                        // have we seen the pattern before?
                        // What we seen so far...
                        let slice = [&pattern[0..idx], &[*a]].concat();
                        let pos = Automat::sigma_suffix(slice.as_slice());
                        if pos != 0 {
                            transition_functions.insert(
                                (idx, *a),
                                pos
                            );
                        }
                        // println!("next: {}\nslice: {}\nposition: {}", a, slice.iter().collect::<String>(), pos);
                    }
                }
            }
            // println!("{:#?}", transition_functions);

            Self{
                accept: pattern.len(),
                transition_functions
            }
        }

        /// Return the length of the longest prefix that is also a suffix
        fn sigma_suffix(x: &[char]) -> usize{
            //P_k is a prefix of P of the length k
            for k in 1..x.len() {
                let prefix = &x[0..x.len()-k];
                let suffix = &x[k..x.len()];
                // println!(
                //     "pre: {}\nsuf: {}",
                //     prefix.iter().collect::<String>(),
                //     suffix.iter().collect::<String>()
                // );
                if prefix == suffix {
                    return prefix.len();
                }
            }
            0
        }
    }

    impl TextMatch for Automat {
        fn match_text(&self, text: &[char]) -> Vec<usize> {
            let mut matches = vec![];
            let mut start = 0;
            for (idx, char) in text.iter().enumerate(){
                start = *self.transition_functions.get(&(start, *char)).unwrap_or(&0);
                if start == self.accept {
                    matches.push(idx-self.accept+1);
                }
            }
            matches
        }
    }

    #[test]
    fn test_sigma() {
        let func = Automat::sigma_suffix;
        let result = func(&['a', 'a', 'b', 'a', 'a', 'a']);
        // println!("longest prefix that is also suffix: {}", result);
        assert_eq!(2, result);
        assert_eq!(1, func(&['a','b','a']));
    }

    #[test]
    fn test_automat() {
        // Automat::new(&"ababbabbababb".chars().collect());
        let auto = Automat::new(&"aabab".chars().collect());
        let result = auto.match_text("aaababaabaababaab".chars().collect::<Vec<char>>().as_slice());
        assert_eq!(vec![1, 9],result);
    }

    pub struct KnuthMorrisPratt{
        pattern: Vec<char>,
        pi: HashMap<usize, usize>
    }

    impl KnuthMorrisPratt{
        pub fn new(pattern: &[char]) -> Self{
            let mut pi = HashMap::new();
            for k in 0..pattern.len() {
                let to = pattern.len()-k;
                let slice = &pattern[0..to];
                let pos = Automat::sigma_suffix(slice);
                println!("k{}, {:?}: {}", to, slice, pos);
                pi.insert(to, pos);
            }
            println!("{:?}", pi);
            Self{
                pattern: pattern.iter().map(|c| *c).collect(),
                pi
            }
        }
    }

    impl TextMatch for KnuthMorrisPratt {
        fn match_text(&self, text: &[char]) -> Vec<usize> {
            let mut matches = vec![];
            let m = self.pattern.len();
            let n = text.len();
            let mut s = 0;
            let mut q = 1;
            while s < n-m {
                println!("s {} q{}, {:?}", s, q, &text[s..s+q]);
                if text[s+q] != self.pattern[q] {
                    let k = self.pi[&q];
                    s += q-k;
                    q = k;
                } else {
                    q += 1;
                    if q == self.pattern.len() {
                        matches.push(s);

                        let k = self.pi[&q];
                        s += q-k;
                        q = k;
                    }
                }
            }
            matches
        }
    }

    #[test]
    fn test_knut() {
        let aaba = KnuthMorrisPratt::new(
            "aabab".chars().collect::<Vec<char>>().as_slice()
        );

        let result = aaba.match_text("aaababaabaababaab".chars().collect::<Vec<char>>().as_slice());
        assert_eq!(vec![1, 9],result);

        let abab = KnuthMorrisPratt::new(
            // "aaababaabaababaab".chars().collect(),
            "abab".chars().collect::<Vec<char>>().as_slice()
        );

        abab.match_text("aaababaabaababaab".chars().collect::<Vec<char>>().as_slice());
    }

    pub struct BoyerMoore;

    impl BoyerMoore{
        pub new()
    }
}

struct SampleSet{
    target: String,
    patterns: Vec<String> 
}

fn main() {
    let results = string_match::rabin_karp(
        "I am sock  söck a sock",
        "sock",
        256u32,
        101u32
    );
    println!("{:?}", results);

    let target_gen = Reader::from_path("./data/gen.fasta").unwrap().records().next().unwrap().unwrap().seq.iter().map(|x| *x as char).collect::<String>();
    let sample_sets = vec![
        SampleSet{
            target: "./data/text.fasta".into(),
            patterns: vec![
                "Besen".into(),
                "Wasserstroeme".into(),
                "Eimer".into()
            ]
        },
        SampleSet{
            target: "./data/Virus.fasta".into(),
            patterns: vec![
                "GTATTA".into(),
                "TTTCGAAA".into(),
                "AAATTGACG".into()
            ]
        },
        SampleSet{
            target: "./data/BA000002.fna".into(),
            patterns: vec![
                "GAATTC".into(),
                "GGATCC".into(),
                "ATTTAAAT".into(),
                target_gen
            ]
        }
    ];

    for sample_set in sample_sets{
        if let Ok(reader) = Reader::from_path(&sample_set.target) {
            for cord in reader.into_records() {
                if let Ok(rec) = cord{
                    let search_source = rec.seq.iter().map(|x| *x as char).collect::<String>();
                    let now = std::time::Instant::now();
                    for word in &sample_set.patterns {
                        let result = string_match::rabin_karp(&search_source, &word, 1u32, 101u32);
                        // println!("Searched: {}, Found at: {:?}", word, result);
                    }
                    println!("Search in '{}' took: {}ms", sample_set.target, now.elapsed().as_millis());
                }
            }
        }
    }

    // if let Ok(reader) = Reader::from_path("./data/text.fasta") {
    //     for cord in reader.into_records() {
    //         if let Ok(rec) = cord{
    //             let search_source = rec.seq.iter().map(|x| *x as char).collect::<String>();
    //             let now = std::time::Instant::now();
    //             for word in &["Besen", "Wasserstroeme", "Eimer"]{
    //                 let result = string_match::rabin_karp(&search_source, &word.chars().collect::<String>(), 2u32, 101u32);
    //                 println!("Searched: {}, Found at: {:?}", word, result);
    //             }
    //             println!("Search took: {}ms", now.elapsed().as_millis());
    //         }
    //     }
    // }
}
