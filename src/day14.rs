use aoc_runner_derive::aoc;

use itertools::Itertools;
use rustc_hash::FxHashMap;

use std::rc::Rc;

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
fn count_after_steps(chain: &[u8], rules: &Rules, steps: i8) -> FxHashMap<u8, usize> {
    let mut counts = FxHashMap::default();

    for p in chain {
        *counts.entry(*p).or_insert(0) += 1;
    }

    let mut memory = FxHashMap::default();

    for (a, b) in chain.iter().tuple_windows() {
        let insertions = insertions_after_steps(&[*a, *b], rules, steps, &mut memory);
        for (k, v) in &*insertions {
            *counts.entry(*k).or_insert(0) += v;
        }
    }

    counts
}

/// Memoized: The insertions done by the given pair after n steps
fn insertions_after_steps(
    pair: &[u8; 2],
    rules: &Rules,
    steps: i8,
    memory: &mut FxHashMap<([u8; 2], i8), Rc<FxHashMap<u8, usize>>>,
) -> Rc<FxHashMap<u8, usize>> {
    if steps == 0 {
        return Rc::new(FxHashMap::default());
    }
    if let Some(mem) = memory.get(&(*pair, steps)) {
        return Rc::clone(mem);
    }

    let mut counts = FxHashMap::default();

    if let Some(c) = rules.get(pair) {
        *counts.entry(*c).or_insert(0) += 1;

        let next_step = steps - 1;
        let left = insertions_after_steps(&[pair[0], *c], rules, next_step, memory);
        let right = insertions_after_steps(&[*c, pair[1]], rules, next_step, memory);

        for (k, v) in &*left {
            *counts.entry(*k).or_insert(0) += v;
        }
        for (k, v) in &*right {
            *counts.entry(*k).or_insert(0) += v;
        }
    }

    let counts = Rc::new(counts);
    memory.insert((*pair, steps), Rc::clone(&counts));
    counts
}

#[aoc(day14, part2)]
pub fn part2(input: &str) -> usize {
    let (template, rules) = parse(input);

    let counts = count_after_steps(template, &rules, 40);

    let most_common = counts.values().max().unwrap();
    let least_common = counts.values().min().unwrap();

    most_common - least_common
}
