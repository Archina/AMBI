pub mod string_match{
    fn str_match(start: usize, text: &str, pattern: &str) -> bool {
        let p_count = pattern.chars().count();
        for p_i in 0..p_count {
            println!("l: {}, r: {}", &text.chars().as_slice()[start+p_i..start+p_i+1], &pattern[p_i..p_i+1]);
            if text[start+p_i..start+p_i+1] != pattern[p_i..p_i+1] {
                continue;
            } else if p_i == p_count - 1{
                return true;
            }
        }
        false
    }

    fn string_sum(text: &str) -> i32 {
        text.as_bytes().iter().map(|byte| *byte as i32).sum::<i32>()
    }

    pub fn naive_match<T,P>(text: T, pattern: P) -> Vec<usize> where T: AsRef<str>, P: AsRef<str> {
        let mut matches= vec![];
        let text = text.as_ref();
        let pattern = pattern.as_ref();
        let t_count = text.chars().count();
        let p_count = pattern.chars().count();
        if p_count > t_count { panic!("pattern is longer than text"); }
        for t_i in 0..t_count-p_count+1 {
            if str_match(t_i, &text, &pattern) {
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

    pub fn rabin_karp<T, H>(text: T, pattern: T, hasher: H) -> Vec<usize>
    where T: AsRef<str>, H: Fn(i32) -> i32 {
        let mut matches= vec![];
        let text = text.as_ref();
        let pattern = pattern.as_ref();
        let p_count = pattern.chars().count();
        let hashed_pattern = hasher(string_sum(pattern));
        println!("{}: {}", "Pattern", hashed_pattern);
        let mut sum_text_slice = string_sum(&text[0..p_count]);
        for (idx, char) in text.chars().skip(p_count-1).enumerate() {
            if idx != 0 {
                if let Some(former) = text.chars().nth(idx-1){
                    let previous = string_sum(&former.to_string());
                    let next = string_sum(&char.to_string());
                    sum_text_slice += next - previous;
                }
            }
            println!("{}: {}", idx, hasher(sum_text_slice));
            if hasher(sum_text_slice) == hashed_pattern && str_match(idx, text, pattern) {
                matches.push(idx);
            }
        }
        matches
    }
}

fn main() {
    let results = string_match::rabin_karp(
        "I am sock  söck a sock",
        "sock",
        |input| input % 16
    );
    println!("{:?}", results);
}
