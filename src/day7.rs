use aoc_runner_derive::aoc;

fn crab_posits(input: &str) -> Vec<i16> {
    input.split(',').map(|n| n.parse().unwrap()).collect()
}

fn linear_fuel_cost(input: &[i16], to: i64) -> i64 {
    input.iter().map(|c| (*c as i64 - to).abs()).sum()
}

fn quadratic_fuel_cost(input: &[i16], to: i64) -> i64 {
    input.iter().map(|c| {
        let dist = (*c as i64 - to).abs();
        // (1..=distance).sum::<i64>()
        (dist * (dist + 1)) / 2
    }).sum()
}

fn median(input: &mut [i16]) -> i16 {
    let midway = input.len() / 2;
    let (_, median, _) = input.select_nth_unstable(midway);
    *median
}

#[aoc(day7, part1)]
pub fn part1(input: &str) -> i64 {
    let mut crabs = crab_posits(input);
    let med = median(&mut crabs) as i64;
    linear_fuel_cost(&crabs, med)
}

#[aoc(day7, part2)]
pub fn part2(input: &str) -> i64 {
    let crabs = crab_posits(input);
    let sum: i64 = crabs.iter().fold(0i64, |acc, c| acc + *c as i64);
    let mean = sum / crabs.len() as i64;
    quadratic_fuel_cost(&crabs, mean)
}
