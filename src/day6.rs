use aoc_runner_derive::{aoc, aoc_generator};

type FishCounts = [i64; 9];

#[aoc_generator(day6)]
pub fn parse_fish(input: &str) -> FishCounts {
    let mut counts = FishCounts::default();
    let nums = input.split(',').map(|n| n.parse::<usize>().unwrap());
    for num in nums {
        counts[num] += 1;
    }
    counts
}

fn fish_after_day(mut counts: FishCounts, day: usize) -> i64 {
    for _day in 0..day {
        let spawn = counts[0];
        counts.rotate_left(1);
        counts[6] += spawn;
    }

    counts.iter().sum()
}

#[aoc(day6, part1)]
pub fn part1(input: &FishCounts) -> i64 {
    fish_after_day(*input, 80)
}

#[aoc(day6, part2)]
pub fn part2(input: &FishCounts) -> i64 {
    fish_after_day(*input, 256)
}
