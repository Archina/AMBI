pub fn hamming(a: &[char], b: &[char]) -> Option<usize> {
    if a.len() != b.len() {
        None
    } else {
        let mut distance = 0;
        for i in 0..a.len() {
            if a[i] != b[i] {
                distance += 1;
            }
        }
        Some(distance)
    }
}

#[test]
fn test_hamming(){
    assert_eq!(None, hamming(&['b','b','a','a','a'], &['a','a','a','a']));
    assert_eq!(Some(2), hamming(&['b','b','a','a'], &['a','a','a','a']));
    assert_eq!(Some(0), hamming(&['b','b','a','a'], &['b','b','a','a']));
}

pub fn levenshtein(a: &[char], b: &[char]) -> usize {
    if b.is_empty() || a.is_empty() {
        0
    } else if b[0] == a[0] {
        levenshtein(&a[1..], &b[1..])
    } else {
        1 + levenshtein(&a[1..], &b[1..]).min(levenshtein(&a[1..], b)).min(levenshtein(a, &b[1..]))
    }
}

#[test]
fn test_leven() {
    let seq_a: Vec<char> = "ATGCAGT".chars().collect();
    let seq_b: Vec<char> = "ATCCAAGT".chars().collect();
    let seq_c: Vec<char> = "AT".chars().collect();
    assert_eq!(2, levenshtein(seq_a.as_slice(), seq_b.as_slice()));
    assert_eq!(0, levenshtein(seq_a.as_slice(), seq_c.as_slice()));
    assert_eq!(0, levenshtein(seq_b.as_slice(), seq_c.as_slice()));
}