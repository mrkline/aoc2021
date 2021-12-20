use aoc_runner_derive::{aoc, aoc_generator};

use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Posit {
    x: i32,
    y: i32,
    z: i32,
}

impl fmt::Display for Posit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

type Readings = Vec<Posit>;

#[aoc_generator(day19)]
fn parse_scanners(input: &str) -> Vec<Readings> {
    let mut lines = input.lines();

    let mut scanners = Vec::new();
    while let Some(readings) = parse_readings(&mut lines) {
        scanners.push(readings);
    }

    scanners
}

fn parse_readings(lines: &mut std::str::Lines<'_>) -> Option<Readings> {
    let header = lines.next();
    match header {
        Some(line) => {
            assert!(line.starts_with("--- scanner "));
            assert!(line.ends_with(" ---"));
        }
        None => return None,
    };

    let mut readings = Vec::new();

    loop {
        match lines.next() {
            Some(l) if l.is_empty() => break,
            Some(l) => readings.push(parse_posit(l)),
            None => break,
        };
    }

    Some(readings)
}

fn parse_posit(line: &str) -> Posit {
    let mut dimensions = line.split(',').map(|s| s.parse().unwrap());
    let x = dimensions.next().unwrap();
    let y = dimensions.next().unwrap();
    let z = dimensions.next().unwrap();
    Posit { x, y, z }
}

#[aoc(day19, part1)]
fn part1(readings: &[Readings]) -> i64 {
    println!("{:#?}", readings);
    42
}
