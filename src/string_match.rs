use std::collections::{HashMap, VecDeque};
use colored::*;
use primes::PrimeSet;

fn str_match<T>(start: usize, text: &[T], pattern: &[T]) -> bool where T: Eq {
    // text[start..start+pattern.len()] == pattern[0..pattern.len()]
    for i in 0..pattern.len() {
        if text[start+i] != pattern[i] {
            return false
        }
    }
    true
}

pub fn naive_match<T,P>(text: T, pattern: P) -> Vec<usize> where T: AsRef<[char]>, P: AsRef<[char]> {
    let mut matches= vec![];
    let text = text.as_ref();
    let pattern = pattern.as_ref();
    let t_count = text.len();
    let p_count = pattern.len();
    if p_count > t_count { panic!("pattern is longer than text"); }
    for t_i in 0..t_count-p_count+1 {
        if str_match(
            t_i,
            text,
            pattern
        ) {
            matches.push(t_i);
        }
    }
    matches
}

#[test]
fn test_naive_matches() {
    let (pattern, text): (Vec<char>, Vec<char>) = ("sock".chars().collect(), "I am sock a sock".chars().collect());
    let results = naive_match(text.as_slice(), pattern.as_slice());
    assert_eq!(vec![5,12], results);
}

#[test]
#[should_panic]
fn test_naive_crash() {
    let (pattern, text): (Vec<char>, Vec<char>) = ("sock tube colour nope".chars().collect(), "I am".chars().collect());
    naive_match(text.as_slice(), pattern.as_slice());
}

pub struct RabinKarp{
    trans_pattern: Vec<u32>,
    sigma_map: HashMap<char, u32>,
    d_base: u32,
    q_prime: u32,
}

impl RabinKarp{
    pub fn new(pattern: &[char], sigma: &[char]) -> Self {
        let sigma_map: HashMap<char, u32> = sigma.iter().enumerate().map(|(idx, char)| (*char, idx as u32)).collect();
        let d_base = (sigma_map.len() - 1) as u32;
        let (_, q_prime) = primes::Sieve::new().find(d_base as u64);
        Self{
            d_base,
            trans_pattern: pattern.iter().map(|p| sigma_map[p]).collect(),
            sigma_map,
            q_prime: q_prime as u32
        }
    }
}

impl TextMatch for RabinKarp {
    fn match_text(&self, text: &[char]) -> Vec<usize> {
        let mut matches= vec![];

        let m = self.trans_pattern.len();

        let trans_text: Vec<u32> = text.iter().map(|p| self.sigma_map[p]).collect();

        let h = self.d_base.pow(m as u32 - 1) % self.q_prime;
        let mut p = 0;
        let mut t_s = 0;

        let mut buffer = VecDeque::<u32>::with_capacity(m+1);

        for (_i, (p_i, t_i)) in self.trans_pattern.iter().zip(trans_text.iter()).enumerate(){
            p = (self.d_base * p + *p_i) % self.q_prime;
            t_s = (self.d_base * t_s + *t_i) % self.q_prime;
            buffer.push_back(*t_i);
        }

        // println!("0: {:?}\n{}", buffer, t_s);
        let mut iter = trans_text.iter().skip(m);
        let mut i = 0;
        loop {
            if t_s == p && str_match(i, trans_text.as_slice(), self.trans_pattern.as_slice()) {
                matches.push(i);
            }
            if let (Some(pop), Some(push)) = (buffer.pop_front(), iter.next()) {
                i += 1;
                // println!("was: {}", t_s);
                // println!("pop: {}",(pop as u32 * h) % q_prime);
                // println!("push: {}",(push as u32) % q_prime);
                let t_next = ((t_s as i32 - (pop * h) as i32) * self.d_base as i32 + *push as i32).rem_euclid(self.q_prime as i32);
                // println!("is: {}", t_next);
                t_s = t_next as u32;
                buffer.push_back(*push);
                // println!("{}: {:?}\n{}", i, buffer, t_s);
            } else {
                break;
            }
        }
        matches
    }
}

pub fn rabin_karp<T>(text: T, pattern: T, d_base: u32, q_prime: u32) -> Vec<usize> where T: AsRef<[char]> {
    let mut matches= vec![];

    let text = text.as_ref();
    let pattern = pattern.as_ref();

    // let n = text.chars().count();
    let m = pattern.len();

    let h = d_base.pow(m as u32 - 1) % q_prime;
    let mut p = 0;
    let mut t_s = 0;

    let mut buffer = VecDeque::<char>::with_capacity(m+1);

    for (_i, (p_i, t_i)) in pattern.iter().zip(text.iter()).enumerate(){
        p = (d_base * p + *p_i as u32) % q_prime;
        t_s = (d_base * t_s + *t_i as u32) % q_prime;
        buffer.push_back(*t_i);
    }

    // println!("0: {:?}\n{}", buffer, t_s);
    let mut iter = text.iter().skip(m);
    let mut i = 0;
    loop {
        if t_s == p {
            // TODO actual string or u match
            matches.push(i);
        }
        if let (Some(pop), Some(push)) = (buffer.pop_front(),iter.next()) {
            i += 1;
            // println!("was: {}", t_s);
            // println!("pop: {}",(pop as u32 * h) % q_prime);
            // println!("push: {}",(push as u32) % q_prime);
            let t_next = ((t_s as i32 - (pop as u32 * h) as i32) * d_base as i32 + (*push as u32) as i32).rem_euclid(q_prime as i32);
            // println!("is: {}", t_next);
            t_s = t_next as u32;
            buffer.push_back(*push);
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
    transition_functions: hashbrown::HashMap<
        (usize, char), usize
    >,
}

impl Automat{
    pub fn new(pattern: &[char]) -> Self{
        let mut alphabet = pattern.to_vec();
        alphabet.sort_unstable();
        alphabet.dedup();
        let mut transition_functions = hashbrown::HashMap::with_capacity(alphabet.len() * pattern.len()+1);
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
            // start = *self.transitions[start].get(char).unwrap_or(&0);
            // self.transition_functions[&(start, *char)];
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
    let slice = "aabab".chars().collect::<Vec<char>>();
    let auto = Automat::new(slice.as_slice());
    let result = auto.match_text("aaababaabaababaab".chars().collect::<Vec<char>>().as_slice());
    assert_eq!(vec![1, 9],result);
}

pub struct KnuthMorrisPratt{
    pattern: Vec<char>,
    pi: HashMap<usize, usize>
}

impl KnuthMorrisPratt{
    pub fn new(pattern: &[char]) -> Self{
        let pi = compute_prefix(pattern);
        // println!("{:?}", pi);
        Self{
            pattern: pattern.iter().map(|c| *c).collect(),
            pi
        }
    }
}

fn compute_prefix(pattern: &[char]) -> HashMap<usize, usize> {
    let mut pi = HashMap::new();
    for k in 0..pattern.len() {
        let to = pattern.len()-k;
        let slice = &pattern[0..to];
        let pos = Automat::sigma_suffix(slice);
        // println!("k{}, {:?}: {}", to, slice, pos);
        pi.insert(to, pos);
    }
    pi
}

impl TextMatch for KnuthMorrisPratt {
    fn match_text(&self, text: &[char]) -> Vec<usize> {
        let mut matches = vec![];
        let m = self.pattern.len();
        let n = text.len();
        let mut s = 0;
        let mut q = 1;
        while s < n-m {
            // println!("s {} q{}, {:?}", s, q, &text[s..s+q]);
            if text[s+q] != self.pattern[q] {
                if None == self.pi.get(&q) {
                    println!("Crashed at s:{} q:{} pi:{:?}", s, q, self.pi);
                }
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

fn last_occurancies(pattern: &[char]) -> HashMap<char, usize> {
    let mut map = HashMap::new();
    for (idx, char) in pattern.iter().enumerate() {
        map.insert(*char, idx);
    }
    map
}

fn good_suffix(pattern: &[char]) -> Vec<usize> {
    let m = pattern.len()-1;
    let pi = compute_prefix(pattern);
    let pattern_ = pattern.iter().copied().rev().collect::<Vec<char>>();
    let pi_ = compute_prefix(pattern_.as_slice());
    let mut lambda = vec![m - pi[&m]; m];
    for l in 1..=m {
        // println!("l:{}, m:{}",l, m);
        let pi_off = pi_[&l];
        let j = m-pi_off-1;
        let next = l - pi_off;
        if lambda[j] > next {
            lambda[j] = next;
        } 
    }
    // println!(
    //     "lambda:{:?}\npi:{:?}\npi_:{:?}\nm:{}",
    //     lambda.iter().map(|i| format!("{}", i)).collect::<String>(),
    //     pi,
    //     pi_,
    //     m
    // );
    lambda
}

pub struct BoyerMoore{
    last_occurancies: HashMap<char, usize>,
    good_suffix: Vec<usize>,
    pattern: Vec<char>
}

impl BoyerMoore{
    pub fn new(pattern: &[char]) -> Self {
        Self{
            last_occurancies: last_occurancies(pattern),
            good_suffix: good_suffix(pattern),
            pattern: pattern.to_vec()
        }
    } 
}

impl TextMatch for BoyerMoore {
    fn match_text(&self, text: &[char]) -> Vec<usize> {
        let mut matches = vec![];
        let (mut s, m, n) = (0, self.pattern.len(), text.len());

        while s < n-m+1 {
            let mut j = m-1;
            let mut t = text[s+j];
            while j > 0 && self.pattern[j] == t {
                j -= 1;
                t = text[s+j];
            }
            // println!(
            //     "(s,j,t):({},{},{})\n{}{}{}{}\n{}{}",
            //     s,j,t,
            //     &text[0..s].iter().collect::<String>(),
            //     &text[s..s+j+1].iter().collect::<String>().as_str().red(),
            //     &text[s+j+1..s+m].iter().collect::<String>().as_str().green(),
            //     &text[s+m..n].iter().collect::<String>(),
            //     vec![' ';s].iter().collect::<String>(),
            //     &pattern.iter().collect::<String>().as_str().blue()
            // );
            s += if j == 0 {
                // println!("Whoa...");
                matches.push(s);
                self.good_suffix[0]
            } else {
                // println!("t:{}, j:{}, l_occ:{}, g_s:{}", t, j, last_occurancies[&t], good_suffix[j-1]);
                let check = (self.last_occurancies.get(&t).and_then(|
                    oc| (oc+1).checked_sub(j))).unwrap_or_default();
                // println!("check: {}", check);
                (self.good_suffix[j-1]).max(check)
            };
            // println!("shift to: {}",s);
            // return matches;
        }
        matches
    }
}

pub fn boyer_moore(text: &[char], pattern: &[char]) -> Vec<usize> {
    let mut matches = vec![];
    let (mut s, m, n) = (0, pattern.len(), text.len());

    let last_occurancies = last_occurancies(pattern);
    let good_suffix = good_suffix(pattern);

    // println!("lo: {:?}, gs:{:?}", last_occurancies, good_suffix);

    while s < n-m+1 {
        let mut j = m-1;
        let mut t = text[s+j];
        while j > 0 && pattern[j] == t {
            j -= 1;
            t = text[s+j];
        }
        // println!(
        //     "(s,j,t):({},{},{})\n{}{}{}{}\n{}{}",
        //     s,j,t,
        //     &text[0..s].iter().collect::<String>(),
        //     &text[s..s+j+1].iter().collect::<String>().as_str().red(),
        //     &text[s+j+1..s+m].iter().collect::<String>().as_str().green(),
        //     &text[s+m..n].iter().collect::<String>(),
        //     vec![' ';s].iter().collect::<String>(),
        //     &pattern.iter().collect::<String>().as_str().blue()
        // );
        s += if j == 0 {
            // println!("Whoa...");
            matches.push(s);
            good_suffix[0]
        } else {
            // println!("t:{}, j:{}, l_occ:{}, g_s:{}", t, j, last_occurancies[&t], good_suffix[j-1]);
            let check = (last_occurancies.get(&t).and_then(|
                oc| (oc+1).checked_sub(j))).unwrap_or_default();
            // println!("check: {}", check);
            (good_suffix[j-1]).max(check)
        };
        // println!("shift to: {}",s);
        // return matches;
    }
    matches
}

#[test]
fn test_boyer_moore() {
    let pattern = "aabab".chars().collect::<Vec<char>>();
    let text = "aaababaabaababaab".chars().collect::<Vec<char>>();
    let bm = BoyerMoore::new(pattern.as_slice());
    let result = bm.match_text(
        text.as_slice()
    );
    assert_eq!(vec![1,9], result);
}

#[test]
fn test_boyer_moore_2() {
    let pattern = "ababababca".chars().collect::<Vec<char>>();
    let text = "aaababaabaababacab".chars().collect::<Vec<char>>();
    let bm = BoyerMoore::new(pattern.as_slice());
    let result = bm.match_text(
        text.as_slice()
    );
    assert_eq!(Vec::<usize>::new(), result);
}

#[test]
fn test_boyer_moore_ueb1() {
    let pattern = "abaabab".chars().collect::<Vec<char>>();
    let text = "abaabaaabaabab".chars().collect::<Vec<char>>();
    let bm = BoyerMoore::new(pattern.as_slice());
    let result = bm.match_text(
        text.as_slice()
    );
    assert_eq!(vec![7], result);
}
