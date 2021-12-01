use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc_generator(day1)]
pub fn intenator(input: &str) -> Vec<u16> {
    input.lines().map(|i| i.parse().unwrap()).collect()
}

#[aoc(day1, part1)]
pub fn part1(input: &[u16]) -> usize {
    input.windows(2).filter(|w| w[1] > w[0]).count()
}

#[aoc(day1, part2)]
pub fn part2(input: &[u16]) -> usize {
    input
        .iter()
        .tuple_windows()
        .map(|(a, b, c)| a + b + c)
        .tuple_windows()
        .filter(|(l, r)| r > l)
        .count()
}
