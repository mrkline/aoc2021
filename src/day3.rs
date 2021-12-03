use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day3)]
pub fn parse_bits(input: &str) -> Vec<u16> {
    input
        .lines()
        .map(|i| u16::from_str_radix(i, 2).unwrap())
        .collect()
}

fn find_gamma(input: &[u16]) -> u16 {
    // 12-bit nums
    let mut bit_counts: [usize; 12] = [0; 12];

    // Count the occurrence of each bit in inputs
    for num in input {
        for (i, count) in bit_counts.iter_mut().enumerate() {
            if (num & (1 << i)) != 0 {
                *count += 1;
            }
        }
    }

    // In gamma, a bit is 1 if that bit position in inputs was commonly 1.
    let mut gamma = 0u16;
    for (i, count) in bit_counts.iter_mut().enumerate() {
        if *count >= input.len() / 2 {
            gamma |= 1 << i;
        }
    }

    gamma
}

#[aoc(day3, part1)]
pub fn part1(input: &[u16]) -> u32 {
    let gamma = find_gamma(input) as u32;
    let epsilon = (!gamma) & 0xfff;
    gamma * epsilon
}

enum Pick {
    Least,
    Most,
}

fn bit_search(input: &[u16], pick: Pick) -> u16 {
    let mut possibles: Vec<u16> = input.to_vec(); // We'll pare these down...
    let mut current_bit: isize = 11; // Start at the MSB

    // While we still have more than one option...
    while possibles.len() > 1 {
        assert!((0..12).contains(&current_bit)); // Sanity check: valid bitmask
        let mask: u16 = 1 << current_bit;

        // Determine bit criteria:

        // What's the common bit?
        let common = possibles.iter().filter(|p| (*p & mask) != 0).count() >= possibles.len() / 2;
        // Do we want the most or least common?
        let criteria = match pick {
            Pick::Most => common,
            Pick::Least => !common,
        };
        // Turn it into a mask to compare against.
        let criteria_mask = if criteria { mask } else { 0 };

        // Filter possibilities based on this bit's criteria.
        let new_possibles = possibles
            .into_iter()
            .filter(|p| (*p & mask) == criteria_mask)
            .collect();

        possibles = new_possibles;
        current_bit -= 1; // Walk towards the LSB
    }
    assert_eq!(possibles.len(), 1); // We'd better have one left.
    possibles[0]
}

#[aoc(day3, part2)]
pub fn part2(input: &[u16]) -> u32 {
    let o2 = bit_search(input, Pick::Most) as u32;
    let scrubber = bit_search(input, Pick::Least) as u32;
    o2 * scrubber
}
