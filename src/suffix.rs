#[derive(Debug)]
enum SuffixNode{
    Edge{label: Vec<char>, childs: Vec<SuffixNode>},
}

#[derive(Debug)]
pub struct SuffixTree{
    childs: Vec<SuffixNode>
}

impl SuffixTree{

    pub fn wotd(input: &[char]) -> Self {
        let mut node = SuffixTree{childs: vec![]};
        let mut sigma = input.to_vec();
        sigma.sort_unstable();
        sigma.dedup();
        SuffixTree::wotd_rec(sigma.as_slice(), SuffixTree::set_of_words(input), &mut node.childs);
        node
    }

    fn wotd_rec(sigma: &[char], set_of_words: Vec<&[char]>, root: &mut Vec<SuffixNode>) {
        for c in sigma{
            let s_c: Vec<&&[char]> = set_of_words.iter().filter(move |w| w.starts_with(&[*c])).collect();
            let count = s_c.len();
            match count{
                0 => (),
                1 => {
                    // Add new edge
                    if let Some(entry) = s_c.first(){
                        root.push(SuffixNode::Edge{
                            label: entry.to_vec(),
                            childs: vec![]
                        });
                    }
                },
                _ => {
                    // Determine longest prefix
                    let prefix = SuffixTree::longest_prefix(&s_c);
                    let mut children = vec![];
                    SuffixTree::wotd_rec(sigma, s_c.iter().filter(|x| x.len() > prefix.len()).map(|c| &c[prefix.len()..]).collect(), &mut children);
                    // Append all longest bois
                    root.push(SuffixNode::Edge{
                        label: prefix,
                        childs: children
                    });
                }
            }
        }
    }

    fn set_of_words(input: &[char]) -> Vec<&[char]> {
        let mut result = vec![];
        for i in 0..input.len() {
            result.push(&input[i..]);
        }
        result
    }

    fn longest_prefix(s_c: &[&&[char]]) -> Vec<char> {
        let mut longest_prefix = vec![s_c[0][0]];
        let mut idx = 1;
        while s_c[0].len() < idx && s_c.iter().skip(1).all(|cs| cs.len() < idx && cs[idx] == s_c[0][idx]) {
            longest_prefix.push(s_c[0][idx]);
            idx += 1;
        }
        longest_prefix
    }
}

#[test]
fn test_set_off_words() {
    let input = vec!['a','b','a','a','b'];
    let set_of_words = SuffixTree::set_of_words(input.as_slice());
    println!("{:?}", set_of_words);
}

#[test]
fn test_longest_prefix(){
    let suf = SuffixTree::longest_prefix(&[
        &vec!['a','b','a','b'].as_slice(),
        &vec!['a','b','a'].as_slice(),
        &vec!['a','b','b'].as_slice()
    ]);
    println!("{:?}", suf);
}

#[test]
fn test_wotd(){
    let st = SuffixTree::wotd(&['a','b','a','b','b']);
    println!("{:#?}", st);
}

fn implicit(){}

fn ukkonen(){}

/// Maximum Unique Matches
fn mum(input: &[char], len: usize) {

}

/// Minimal Unique Substring
fn mus(input: &[char], len: usize) {

}