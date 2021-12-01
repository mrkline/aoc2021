use aoc_runner_derive::aoc;
use itertools::Itertools;

#[aoc(day1, part1)]
pub fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|i| i.parse::<u16>().unwrap())
        .tuple_windows()
        .filter(|(l, r)| r > l)
        .count()
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> usize {
    input
        .lines()
        .map(|i| i.parse::<u16>().unwrap())
        .tuple_windows()
        .map(|(a, b, c)| a + b + c)
        .tuple_windows()
        .filter(|(l, r)| r > l)
        .count()
}
