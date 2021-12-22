#![allow(unused)]
use aoc_runner_derive::{aoc, aoc_generator};

use nalgebra as na;
use std::fmt;

type Posit = na::geometry::Point3<i32>;

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

    // readings.sort_unstable_by_key(|v| (v.x, v.y, v.z));
    Some(readings)
}

fn parse_posit(line: &str) -> Posit {
    let mut dimensions = line.split(',').map(|s| s.parse().unwrap());
    let x = dimensions.next().unwrap();
    let y = dimensions.next().unwrap();
    let z = dimensions.next().unwrap();
    Posit::new(x, y, z)
}

type Rotation = na::Matrix3<i32>;

#[rustfmt::skip]
fn roll(m: &Rotation) -> Rotation {
    // def roll(v): return (v[0],v[2],-v[1])
    const ROLLER: Rotation = Rotation::new(
        1, 0, 0,
        0, 0, 1,
        0, -1, 0
    );
    m * ROLLER
}

#[rustfmt::skip]
fn turn(m: &Rotation) -> Rotation {
    // def turn(v): return (-v[1],v[0],v[2])
    const TURNER: Rotation = Rotation::new(
        0, -1, 0,
        1, 0, 0,
        0, 0, 1
    );
    m * TURNER
}

/// Produce all 24 rotation matrices around three axes
///
/// https://stackoverflow.com/a/16467849
fn all_axis_rotations() -> Vec<Rotation> {
    // def sequence (v):
    //     for cycle in range(2):
    //         for step in range(3):  # Yield RTTT 3 times
    //             v = roll(v)
    //             yield(v)           #    Yield R
    //             for i in range(3): #    Yield TTT
    //                 v = turn(v)
    //                 yield(v)
    //         v = roll(turn(roll(v)))  # Do RTR
    let mut rotations = Vec::with_capacity(24);
    let mut v = Rotation::identity();

    for _ in 0..2 {
        for _ in 0..3 {
            rotations.push(v);
            v = roll(&v);
            for _ in 0..3 {
                rotations.push(v);
                v = turn(&v);
            }
        }
        v = roll(&turn(&roll(&v)));
    }

    assert_eq!(rotations.len(), 24);
    rotations
}

fn nth_rotation(n: usize) -> &'static Rotation {
    lazy_static::lazy_static! {
        static ref ROTATIONS: Vec<Rotation> = all_axis_rotations();
    }
    &ROTATIONS[n]
}

#[aoc(day19, part1)]
fn part1(readings: &[Readings]) -> i64 {
    42
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn rotations() {
        let input = r"--- scanner 0 ---
-1,-1,1
-2,-2,2
-3,-3,3
-2,-3,1
5,6,-4
8,0,7

--- scanner 0 ---
1,-1,1
2,-2,2
3,-3,3
2,-1,3
-5,4,-6
-8,-7,0

--- scanner 0 ---
-1,-1,-1
-2,-2,-2
-3,-3,-3
-1,-3,-2
4,6,5
-7,0,8

--- scanner 0 ---
1,1,-1
2,2,-2
3,3,-3
1,3,-2
-4,-6,5
7,0,8

--- scanner 0 ---
1,1,1
2,2,2
3,3,3
3,1,2
-6,-4,-5
0,7,-8";

        let scanners = parse_scanners(input);

        let first_scanner = &scanners[0];

        for s in scanners.iter().skip(1) {
            // Find the rotation that gives us the nth orientation
            let right_rotation: &Rotation = (0..24)
                .map(nth_rotation)
                .find(|r| (*r * s.last().unwrap()) == *first_scanner.last().unwrap())
                .unwrap();

            for (first, reoriented) in first_scanner.iter().zip(s.iter()) {
                assert_eq!(*first, right_rotation * reoriented);
            }
        }
    }
}
