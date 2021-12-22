#![allow(unused)]
use aoc_runner_derive::{aoc, aoc_generator};

use std::fmt;
use nalgebra as na;

type Posit = na::Vector3<i32>;

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

    readings.sort_unstable_by_key(|v| (v.x, v.y, v.z));
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
    lazy_static::lazy_static!{
        static ref ROTATIONS: Vec<Rotation> = all_axis_rotations();
    }
    &ROTATIONS[n]
}

#[aoc(day19, part1)]
fn part1(readings: &[Readings]) -> i64 {
    // println!("{:#?}", readings);
    for r in all_axis_rotations() {
        print!("{}", r);
    }
    42
}

#[cfg(test)]
mod test {
    use super::*;
}
