use aoc_runner_derive::{aoc, aoc_generator};

use std::fmt::{Debug, Error, Formatter};

#[derive(Default, Clone)]
pub struct Map {
    cells: Vec<i8>,
    width: usize,
    height: usize,
}

type Coordinate = (usize, usize);

impl Map {
    fn risk(&self, c: Coordinate) -> i8 {
        let x = c.0;
        let y = c.1;
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

fn a_star(map: &Map) -> Score {
    use pathfinding::directed::astar::astar;

    const START: Coordinate = (0, 0);
    let end: Coordinate = (map.width - 1, map.height - 1);

    let successors = |c: &Coordinate| {
        let neighbors = [
            (c.0 - 1, c.1),
            (c.0 + 1, c.1),
            (c.0, c.1 - 1),
            (c.0, c.1 + 1),
        ];
        neighbors
            .into_iter()
            .filter(|(nx, ny)| *nx < map.width && *ny < map.height)
            .map(|n| (n, map.risk(n) as Score))
    };

    let heuristic = |c: &Coordinate| -> Score {
        let dy = c.1 as Score - map.height as Score;
        let dx = c.0 as Score - map.width as Score;
        // Assume the average risk is 5.
        (dx + dy) * 5
    };

    let search_result = astar(&START, successors, heuristic, |c| *c == end);
    search_result.expect("No path to end").1
}

#[aoc(day15, part1)]
pub fn part1(input: &Map) -> Score {
    a_star(input)
}

#[aoc(day15, part2)]
pub fn part2(input: &Map) -> Score {
    let width = input.width * 5;
    let height = input.height * 5;
    let mut cells = Vec::with_capacity(input.cells.len() * 5 * 5);

    for dup_y in 0..5 {
        for y in 0..input.height {
            for dup_x in 0..5 {
                let dup = dup_y + dup_x;
                for x in 0..input.width {
                    let mut new_val = input.risk((x, y)) + dup;
                    if new_val > 9 {
                        new_val -= 9;
                    }
                    cells.push(new_val);
                }
            }
        }
    }

    let embiggened = Map {
        cells,
        width,
        height,
    };
    a_star(&embiggened)
}
