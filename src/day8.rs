use aoc_runner_derive::aoc;

use std::fmt::{Debug, Error, Formatter};

use rustc_hash::FxHashMap;

fn outputs_only(line: &str) -> impl Iterator<Item = &str> {
    let mut split_iter = line.split(" | ");
    let _segs = split_iter.next();
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

fn to_seg(segs: &str) -> u8 {
    segs.as_bytes()
        .iter()
        .copied()
        .map(to_bit)
        .fold(0u8, |acc, l| acc | l)
}

#[derive(Default)]
struct Display {
    segs: [u8; 10],
    outputs: [u8; 4],
}

fn seg_string(seg: u8) -> String {
    let mut s = String::new();

    for i in 0..8 {
        if seg & (1 << i) != 0 {
            s.push(char::from_u32(b'a' as u32 + i).unwrap());
        }
    }

    s
}

impl Debug for Display {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        for seg in self.segs {
            write!(f, "{} ", seg_string(seg))?;
        }
        write!(f, "| ")?;
        writeln!(
            f,
            "{} {} {} {}",
            seg_string(self.outputs[0]),
            seg_string(self.outputs[1]),
            seg_string(self.outputs[2]),
            seg_string(self.outputs[3])
        )
    }
}

fn parse_line(line: &str) -> Display {
    let mut display = Display::default();

    let mut split_iter = line.split(" | ");
    let segs = split_iter.next().unwrap();
    let outputs = split_iter.next().unwrap();

    for (i, seg) in segs.split_ascii_whitespace().map(to_seg).enumerate() {
        display.segs[i] = seg;
    }

    for (i, output) in outputs.split_ascii_whitespace().map(to_seg).enumerate() {
        display.outputs[i] = output;
    }

    assert!(display.segs.iter().all(|s| *s != 0));
    assert!(display.outputs.iter().all(|o| *o != 0));
    display
}

type Mapping = FxHashMap<u8, u8>;

fn find_mapping(segs: &[u8]) -> Mapping {

    //   0:      1:      2:      3:      4:
    //  aaaa    ....    aaaa    aaaa    ....
    // b    c  .    c  .    c  .    c  b    c
    // b    c  .    c  .    c  .    c  b    c
    //  ....    ....    dddd    dddd    dddd
    // e    f  .    f  e    .  .    f  .    f
    // e    f  .    f  e    .  .    f  .    f
    //  gggg    ....    gggg    gggg    ....
    //
    //   5:      6:      7:      8:      9:
    //  aaaa    aaaa    aaaa    aaaa    aaaa
    // b    .  b    .  .    c  b    c  b    c
    // b    .  b    .  .    c  b    c  b    c
    //  dddd    dddd    ....    dddd    dddd
    // .    f  e    f  .    f  e    f  .    f
    // .    f  e    f  .    f  e    f  .    f
    //  gggg    gggg    ....    gggg    gggg

    // 1 uses 2 segs
    // 4 uses 4 segs
    // 7 uses 3 segs
    // 8 uses 7 segs
    let one = *segs.iter().find(|s| s.count_ones() == 2).unwrap();
    let four = *segs.iter().find(|s| s.count_ones() == 4).unwrap();
    let seven = *segs.iter().find(|s| s.count_ones() == 3).unwrap();
    let eight = *segs.iter().find(|s| s.count_ones() == 7).unwrap();

    // XOR all AND one, i.e., (c, f) is f
    let f = segs.iter().fold(0u8, |acc, s| acc ^ s) & one;

    // f XOR one (c, f) is c
    let c = f ^ one;

    // 6 is 8 XOR c
    let six = eight ^ c;

    // 2, 3, 5 have five segments
    let mut two_three_five = segs.iter().filter(|s| s.count_ones() == 5);

    // 3 is the one with c and f
    let three = *two_three_five.find(|s| *s & one == one).unwrap();

    // 9 is 3 | 4
    let nine = three | four;

    // 0, 6, and 9 all contain six segments. We know 6 and 9, so we know 0
    let zero = *segs
        .iter()
        .find(|s| s.count_ones() == 6 && **s != six && **s != nine)
        .unwrap();

    // 5 is 9 ^ c
    let five = nine ^ c;

    // 2 is (3 | !5) & !f
    let two = (three | !five) & !f & 0b0111_1111;

    let mut mapping = Mapping::default();
    mapping.reserve(10);
    mapping.insert(zero, 0);
    mapping.insert(one, 1);
    mapping.insert(two, 2);
    mapping.insert(three, 3);
    mapping.insert(four, 4);
    mapping.insert(five, 5);
    mapping.insert(six, 6);
    mapping.insert(seven, 7);
    mapping.insert(eight, 8);
    mapping.insert(nine, 9);

    mapping
}

fn readout(output: &[u8; 4], mapping: &Mapping) -> i64 {
    let thousands = mapping[&output[0]] as i64 * 1000;
    let hundreds = mapping[&output[1]] as i64 * 100;
    let tens = mapping[&output[2]] as i64 * 10;
    let ones = mapping[&output[3]] as i64;

    thousands + hundreds + tens + ones
}

#[aoc(day8, part2)]
pub fn part2(input: &str) -> i64 {
    input
        .lines()
        .map(parse_line)
        .map(|d| {
            let map = find_mapping(&d.segs);
            readout(&d.outputs, &map)
        })
        .sum()
}
