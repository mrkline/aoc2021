use aoc_runner_derive::aoc;
use rustc_hash::FxHashMap;

use std::mem;

#[derive(Debug, Eq, PartialEq, Hash)]
struct Point {
    x: i16,
    y: i16,
}

#[derive(Debug)]
pub struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn is_horizontal(&self) -> bool {
        self.start.y == self.end.y
    }

    fn is_vertical(&self) -> bool {
        self.start.x == self.end.x
    }

    fn is_angled(&self) -> bool {
        !(self.is_horizontal() || self.is_vertical())
    }

    /// Make sure line goes small -> large coord, simplifies things.
    /// Would probably be cleaner to return a new Line instead of mutating...
    fn sort(&mut self) {
        if self.is_vertical() {
            if self.start.y > self.end.y {
                mem::swap(&mut self.start.y, &mut self.end.y);
            }
        } else {
            // Let's at least make sure the line goes left to right
            if self.start.x > self.end.x {
                mem::swap(&mut self.start, &mut self.end);
            }
        }
    }
}

fn parse_point(input: &str) -> Point {
    let mut vals = input.split(',').map(|p| p.parse().unwrap());
    let x = vals.next().unwrap();
    let y = vals.next().unwrap();
    Point { x, y }
}

fn parse_line(input: &str) -> Line {
    let mut pairs = input.split(" -> ");
    let start = parse_point(pairs.next().unwrap());
    let end = parse_point(pairs.next().unwrap());
    Line { start, end }
}

#[aoc(day5, part1)]
pub fn part1(input: &str) -> usize {
    let hv_lines: Vec<Line> = input
        .lines()
        .map(parse_line)
        .filter_map(|mut l| {
            if l.is_angled() {
                None
            } else {
                l.sort();
                Some(l)
            }
        })
        .collect();

    // We could construct a bounding box, then allocate counts for every point,
    // but assume the area is fairly sparse and just use a hashmap of points instead.
    let mut counts: FxHashMap<Point, u16> = FxHashMap::default();
    for line in &hv_lines {
        if line.is_horizontal() {
            let y = line.start.y;
            for x in line.start.x..=line.end.x {
                *counts.entry(Point { x, y }).or_insert(0) += 1;
            }
        } else {
            assert!(line.is_vertical());
            let x = line.start.x;
            for y in line.start.y..=line.end.y {
                *counts.entry(Point { x, y }).or_insert(0) += 1;
            }
        }
    }

    counts.values().filter(|overlaps| **overlaps >= 2).count()
}

#[aoc(day5, part2)]
pub fn part2(input: &str) -> usize {
    let lines: Vec<Line> = input
        .lines()
        .map(parse_line)
        .map(|mut l| {
            l.sort();
            l
        })
        .collect();

    // Ditto
    let mut counts: FxHashMap<Point, u16> = FxHashMap::default();
    for line in &lines {
        if line.is_vertical() {
            let x = line.start.x;
            for y in line.start.y..=line.end.y {
                *counts.entry(Point { x, y }).or_insert(0) += 1;
            }
        } else {
            let mut y = line.start.y;
            let direction = (line.end.y - line.start.y).signum();
            for x in line.start.x..=line.end.x {
                *counts.entry(Point { x, y }).or_insert(0) += 1;
                y += direction;
            }
            assert_eq!(y, line.end.y + direction);
        }
    }

    counts.values().filter(|overlaps| **overlaps >= 2).count()
}
