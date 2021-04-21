pub mod string_match{
    use std::collections::VecDeque;

    fn str_match(start: usize, text: &[char], pattern: &[char]) -> bool {
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
}

fn main() {
    let results = string_match::rabin_karp(
        "I am sock  söck a sock",
        "sock",
        256u32,
        101u32
    );
    println!("{:?}", results);
}
