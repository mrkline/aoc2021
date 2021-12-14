use aoc_runner_derive::aoc;

use itertools::Itertools;
use rustc_hash::FxHashMap;

use std::sync::atomic::{AtomicUsize, Ordering};

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

fn count(vals: &[u8]) -> FxHashMap<u8, usize> {
    let mut counts = FxHashMap::default();

    for v in vals {
        *counts.entry(*v).or_insert(0) += 1;
    }

    counts
}

#[aoc(day14, part1)]
pub fn part1(input: &str) -> usize {
    let (template, rules) = parse(input);
    let mut chain = template.to_vec();
    for _ in 0..10 {
        chain = step(&chain, &rules);
    }

    let counts = count(&chain);

    let most_common = counts.values().max().unwrap();
    let least_common = counts.values().min().unwrap();

    most_common - least_common
}

// Again, but with less memory
fn count_after_steps(chain: &[u8], rules: &Rules, counts: &mut FxHashMap<u8, AtomicUsize>, steps_remaining: isize) {
    for p in chain {
        counts.entry(*p).or_default().fetch_add(1, Ordering::Relaxed);
    }
    for r in rules.values() {
        counts.entry(*r).or_default();
    }

    for (a, b) in chain.iter().tuple_windows() {
        count_after_steps_rec(*a, *b, rules, counts, steps_remaining);
    }
}

fn count_after_steps_rec(a: u8, b: u8, rules: &Rules, counts: &FxHashMap<u8, AtomicUsize>, steps_remaining: isize) {
    if steps_remaining == 0 {
        return;
    }

    let next_step = steps_remaining - 1;

    if let Some(c) = rules.get(&[a, b]) {
        counts.get(c).unwrap().fetch_add(1, Ordering::Relaxed);
        rayon::join(
         || count_after_steps_rec(a, *c, rules, counts, next_step),
         || count_after_steps_rec(*c, b, rules, counts, next_step)
        );
    }
}

#[aoc(day14, part2)]
pub fn part2(input: &str) -> usize {
    let (template, rules) = parse(input);

    let mut counts = FxHashMap::default();
    count_after_steps(template, &rules, &mut counts, 40);

    let most_common = counts.values().map(|v| v.load(Ordering::SeqCst)).max().unwrap();
    let least_common = counts.values().map(|v| v.load(Ordering::SeqCst)).min().unwrap();

    most_common - least_common
}
