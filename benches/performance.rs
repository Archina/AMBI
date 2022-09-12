use ambi::string_match::{Automat, BoyerMoore, TextMatch, naive_match};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

const GENOME: &str = include_str!("genome.in");
const GENE: &str = include_str!("gene.in");

pub fn criterion_bench_automaton(c: &mut Criterion) {
    let pattern = GENE.chars().collect::<Vec<char>>();
    let text = GENOME.chars().collect::<Vec<char>>();
    let short_p = "TTATTAT".chars().collect::<Vec<char>>();
    let auto_long = Automat::new(pattern.as_slice());
    let auto_short = Automat::new(short_p.as_slice());
    c.bench_function("Automaton - Long P", |b| b.iter(|| auto_long.match_text(black_box(text.as_slice()))));
    c.bench_function("Automaton - Short P", |b| b.iter(|| auto_short.match_text(black_box(text.as_slice()))));
}

pub fn criterion_bench_naive(c: &mut Criterion) {
    let pattern = GENE.chars().collect::<Vec<char>>();
    let text = GENOME.chars().collect::<Vec<char>>();
    let short_p = "TTATTAT".chars().collect::<Vec<char>>();
    c.bench_function("Naive - Long P", |b| b.iter(|| naive_match(text.as_slice(), black_box(pattern.as_slice()))));
    c.bench_function("Naive - Short P", |b| b.iter(|| naive_match(text.as_slice(), black_box(short_p.as_slice()))));
}

pub fn criterion_bench_boyer_moore(c: &mut Criterion) {
    let pattern_long = GENE.chars().collect::<Vec<char>>();
    let text = GENOME.chars().collect::<Vec<char>>();
    let pattern_short = "TTATTAT".chars().collect::<Vec<char>>();
    let bm_long = BoyerMoore::new(pattern_long.as_slice());
    let bm_short = BoyerMoore::new(pattern_short.as_slice());
    c.bench_function("Boyer Moore - Long P", |b| b.iter(|| bm_long.match_text(black_box(text.as_slice()))));
    c.bench_function("Boyer Moore - Short P", |b| b.iter(|| bm_short.match_text(black_box(text.as_slice()))));
}

criterion_group!(benches, criterion_bench_naive, criterion_bench_automaton, criterion_bench_boyer_moore);
criterion_main!(benches);
