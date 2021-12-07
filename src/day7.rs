use aoc_runner_derive::aoc;

pub fn crab_posits(input: &str) -> Vec<i16> {
    input.split(',').map(|n| n.parse().unwrap()).collect()
}

fn median(input: &mut [i16]) -> i16 {
    let midway = input.len() / 2;
    let (_, median, _) = input.select_nth_unstable(midway);
    *median
}

fn linear_fuel_cost(input: &[i16], to: i16) -> i64 {
    input.iter().map(|c| (c - to).abs() as i64).sum()
}

fn quadratic_fuel_cost(input: &[i16], to: i16) -> i64 {
    input.iter().map(|c| {
        let dist = (c - to).abs() as i64;
        // (1..=distance).sum::<i64>()
        (dist * (dist + 1)) / 2
    }).sum()
}

#[aoc(day7, part1)]
pub fn part1(input: &str) -> i64 {
    let mut crabs = crab_posits(input);
    let med = median(&mut crabs);
    linear_fuel_cost(&crabs, med)
}

#[aoc(day7, part2)]
pub fn part2(input: &str) -> i64 {
    // let crabs = vec![16,1,2,0,4,2,7,1,2,14];
    let crabs = crab_posits(input);
    let min = crabs.iter().min_by_key(|c| quadratic_fuel_cost(&crabs, **c)).unwrap();
    // let sum = crabs.iter().fold(0i64, |acc, c| acc + *c as i64);
    // let mean = (sum as f64 / crabs.len() as f64).round() as i16;
    quadratic_fuel_cost(&crabs, *min)
}
