use aoc_runner_derive::aoc;

fn outputs_only(line: &str) -> impl Iterator<Item = &str> {
    let mut split_iter = line.split(" | ");
    let _inputs = split_iter.next();
    let outputs = split_iter.next().unwrap();
    outputs.split_ascii_whitespace()
}

fn is_unique_digit(segs: &str) -> bool {
    // Since we only care about digits with a unique # of segments
    // (1, 4, 7, 8), just sum outputs with those values.
    // 1 uses 2 segs
    // 4 uses 4 segs
    // 7 uses 3 segs
    // 8 uses 7 segs
    matches!(segs.len(), 2 | 3 | 4 | 7)
}

#[aoc(day8, part1)]
pub fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|l| outputs_only(l).filter(|out| is_unique_digit(out)).count())
        .sum()
}

fn to_bit(letter: u8) -> u8 {
    1 << (letter - b'a')
}

#[aoc(day8, part2)]
pub fn part2(input: &str) -> usize {
    println!("{:07b}", to_bit(b'a'));
    println!("{:07b}", to_bit(b'b'));
    println!("{:07b}", to_bit(b'c'));
    println!("{:07b}", to_bit(b'd'));
    println!("{:07b}", to_bit(b'e'));
    println!("{:07b}", to_bit(b'f'));
    println!("{:07b}", to_bit(b'g'));
    42
}
