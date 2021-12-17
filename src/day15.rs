use aoc_runner_derive::{aoc, aoc_generator};

use rustc_hash::{FxHashMap, FxHashSet};

use std::fmt::{Debug, Error, Formatter};

#[derive(Default, Clone)]
pub struct Map {
    cells: Vec<i8>,
    width: usize,
    height: usize,
}

type Coordinate = (i8, i8);

impl Map {
    fn risk(&self, c: Coordinate) -> i8 {
        let x = c.0 as usize;
        let y = c.1 as usize;
        assert!(x < self.width);
        assert!(y < self.height);
        self.cells[x + y * self.width]
    }
}

impl Debug for Map {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        for line in self.cells.chunks(self.width) {
            for val in line {
                write!(f, "{}", val)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[aoc_generator(day15)]
pub fn parse(input: &str) -> Map {
    /*
        let input = r"1163751742
    1381373672
    2136511328
    3694931569
    7463417111
    1319128137
    1359912421
    3125421639
    1293138521
    2311944581";
        */

    let mut cells: Vec<i8> = Vec::new();
    let mut width: usize = 0;
    let mut height: usize = 0;

    for line in input.lines() {
        let bytes = line.as_bytes();
        width = bytes.len();
        height += 1;

        for b in bytes {
            cells.push((b - b'0') as i8);
        }
    }

    Map {
        cells,
        width,
        height,
    }
}

type Score = i32;

/// Once we have a path to the end, follow links back to the start
/// to find the cost of the entire path.
fn backtrace_from(
    map: &Map,
    came_from: &FxHashMap<Coordinate, Coordinate>,
    current: Coordinate,
) -> Score {
    if let Some(whence) = came_from.get(&current) {
        let prev_costs = backtrace_from(map, came_from, *whence);
        // println!("({}, {})", current.0, current.1);
        let cost = map.risk(current) as Score;
        prev_costs + cost
    } else {
        0
    }
}

/// Find the lowest-cost node in the open set, pop it, and return it.
///
/// Ideally this would be a min heap, but std::collectoins::BinaryHeap can't
/// take some comparator function - items have to be Ord themselves.
/// This would mean both:
/// 1. Some boilerplate for each item in the set to impl Ord based on the score below
/// 2. Each item in the set would need some references to map, scores, or a Fn that referenced them,
/// 3. Sad mutable borrow issues arising from pt 2.
fn pop_open_set(
    open_set: &mut FxHashSet<Coordinate>,
    map: &Map,
    scores: &FxHashMap<Coordinate, Score>,
) -> Option<Coordinate> {
    let smallest = open_set.iter().copied().min_by_key(|c| {
        // A* heuristic: Distance to the end node
        let dy = (c.1 - map.height as i8) as f64;
        let dx = (c.0 - map.width as i8) as f64;
        let hypot = dx.hypot(dy) as Score;

        (*scores.get(c).unwrap_or(&Score::MAX)).saturating_add(hypot)
    });

    if let Some(smol) = smallest {
        open_set.remove(&smol);
        Some(smol)
    } else {
        None
    }
}

fn a_star(map: &Map) -> Score {
    let mut scores = FxHashMap::default();
    scores.insert((0, 0), 0);

    let mut came_from: FxHashMap<Coordinate, Coordinate> = FxHashMap::default();

    let mut open_set: FxHashSet<Coordinate> = FxHashSet::default();
    open_set.insert((0, 0));

    let end = ((map.width - 1) as i8, (map.height - 1) as i8);

    while let Some(current) = pop_open_set(&mut open_set, map, &scores) {
        // println!("At ({}, {})", current.0, current.1);

        if current == end {
            return backtrace_from(map, &came_from, current);
        }

        let current_score = *scores.get(&current).unwrap_or(&Score::MAX);

        let neighbors = [
            (current.0 - 1, current.1),
            (current.0 + 1, current.1),
            (current.0, current.1 - 1),
            (current.0, current.1 + 1),
        ];

        for neighbor in neighbors.into_iter().filter(|(nx, ny)| {
            *nx >= 0 && *ny >= 0 && *nx < map.width as i8 && *ny < map.height as i8
        }) {
            let risk = map.risk(neighbor) as Score;

            // println!("Neighbor ({}, {}) is {} + {}", neighbor.0, neighbor.1, current_score, risk);
            let neighbor_score_through_current = current_score + risk;
            let current_neighbor_score = *scores.get(&neighbor).unwrap_or(&Score::MAX);

            if neighbor_score_through_current < current_neighbor_score {
                came_from.insert(neighbor, current);
                scores.insert(neighbor, neighbor_score_through_current);
                open_set.insert(neighbor);
            }
        }
    }

    unreachable!("No path to the end!");
}

#[aoc(day15, part1)]
pub fn part1(input: &Map) -> Score {
    println!("{:?}", input);
    a_star(input)
}

#[aoc(day15, part2)]
pub fn part2(input: &Map) -> usize {
    42
}
