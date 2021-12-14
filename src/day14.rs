use aoc_runner_derive::aoc;

use itertools::Itertools;
use rustc_hash::FxHashMap;

type Rules = FxHashMap<[u8; 2], u8>;

pub fn parse(input: &str) -> (&[u8], Rules) {
    let mut lines = input.lines();

    let template = lines.next().unwrap().as_bytes();
    assert_eq!(lines.next(), Some(""));

    let mut rules = FxHashMap::default();

    for line in lines {
        let mut toks = line.split(" -> ");
        let from = toks.next().unwrap().as_bytes().try_into().unwrap();
        let to = toks.next().unwrap().as_bytes()[0];
        rules.insert(from, to);
    }

    (template, rules)
}

fn step(before: &[u8], rules: &Rules) -> Vec<u8> {
    let mut after = Vec::with_capacity(before.len());

    for (a, b) in before.iter().tuple_windows() {
        after.push(*a);
        if let Some(c) = rules.get(&[*a, *b]) {
            after.push(*c);
        }
    }
    after.push(*before.last().unwrap());

    after
}

#[aoc(day14, part1)]
pub fn part1(_input: &str) -> usize {
    let input = r"NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";

    let (template, rules) = parse(input);
    let mut template = template.to_vec();
    for _ in 0..=5 {
        println!(
            "{}: {}",
            template.len(),
            std::str::from_utf8(&template).unwrap()
        );
        template = step(&template, &rules);
    }
    42
}

#[aoc(day14, part2)]
pub fn part2(input: &str) -> usize {
    let (_template, _rules) = parse(input);
    42
}
