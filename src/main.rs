pub mod string_match{
    fn str_match(start: usize, text: &str, pattern: &str) -> Option<usize> {
        let p_count = pattern.chars().count();
        for p_i in 0..p_count {
            if text[start+p_i..start+p_i+1] != pattern[p_i..p_i+1] {
                return None;
            } else if p_i == p_count - 1{
                return Some(start);
            }
        }
        None
    }

    pub fn naive_match<T,P>(text: T, pattern: P) -> Vec<usize> where T: AsRef<str>, P: AsRef<str> {
        let mut matches= vec![];
        let text = text.as_ref();
        let pattern = pattern.as_ref();
        let t_count = text.chars().count();
        let p_count = pattern.chars().count();
        if p_count > t_count { panic!("pattern is longer than text"); }
        for t_i in 0..t_count-p_count+1 {
            if let Some(s) = str_match(t_i, &text, &pattern) {
                matches.push(s);
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

    pub fn rabin_karp<T, H>(text: T, pattern: T, hasher: H) -> Vec<usize>
    where T: AsRef<str>, H: Fn(&str) -> u32 {
        let mut matches= vec![];
        let text = text.as_ref();
        let pattern = pattern.as_ref();
        let t_count = text.chars().count();
        let p_count = pattern.chars().count();
        let hashed_pattern = hasher(pattern);
        for t_i in 0..t_count-p_count+1 {
            // Sliding window?
            let text_slice = &text[t_i..t_i+p_count];
            if hasher(text_slice) == hashed_pattern && str_match(0, text_slice, pattern).is_some() {
                matches.push(t_i);
            }
        }
        matches
    }
}

fn main() {
    let results = string_match::rabin_karp(
        "I am sock a sock",
        "sock",
        |input| input.to_string().as_bytes().iter().map(|byte| *byte as u32).sum::<u32>() % 16
    );
    println!("{:?}", results);
}
