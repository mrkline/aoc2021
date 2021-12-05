use aoc_runner_derive::aoc;
use rustc_hash::FxHashMap;

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
    fn is_angled(&self) -> bool {
        !(self.start.x == self.end.x || self.start.y == self.end.y)
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

fn overlaps<L: Iterator<Item = Line>>(lines: L) -> usize {
    // We could construct a bounding box, then allocate counts for every point,
    // but assume the area is fairly sparse and just use a hashmap of points instead.
    let mut counts: FxHashMap<Point, u16> = FxHashMap::default();
    for line in lines {
        let mut x = line.start.x;
        let mut y = line.start.y;
        let dx = (line.end.x - line.start.x).signum();
        let dy = (line.end.y - line.start.y).signum();
        while y != line.end.y + dy || x != line.end.x + dx {
            *counts.entry(Point { x, y }).or_insert(0) += 1;
            x += dx;
            y += dy;
        }
    }

    counts.values().filter(|overlaps| **overlaps >= 2).count()
}

#[aoc(day5, part1)]
pub fn part1(input: &str) -> usize {
    let hv_lines = input.lines().map(parse_line).filter(|l| !l.is_angled());
    overlaps(hv_lines)
}

#[aoc(day5, part2)]
pub fn part2(input: &str) -> usize {
    let lines = input.lines().map(parse_line);
    overlaps(lines)
}
