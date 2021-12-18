use aoc_runner_derive::{aoc, aoc_generator};

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

fn a_star(map: &Map) -> Score {
    use pathfinding::directed::astar::astar;

    const START: Coordinate = (0, 0);
    let end: Coordinate = ((map.width - 1) as i8, (map.height - 1) as i8);

    let successors = |c: &Coordinate| {
        let neighbors = [
            (c.0 - 1, c.1),
            (c.0 + 1, c.1),
            (c.0, c.1 - 1),
            (c.0, c.1 + 1),
        ];
        neighbors
            .into_iter()
            .filter(|(nx, ny)| {
                *nx >= 0 && *ny >= 0 && *nx < map.width as i8 && *ny < map.height as i8
            })
            .map(|n| (n, map.risk(n) as Score))
    };

    let average_risk: i64 =
        map.cells.iter().map(|c| *c as i64).sum::<i64>() / map.cells.len() as i64;

    let heuristic = |c: &Coordinate| -> Score {
        let dy = (c.1 - map.height as i8) as f64;
        let dx = (c.0 - map.width as i8) as f64;
        (dx.hypot(dy) / average_risk as f64) as Score
    };

    let search_result = astar(&START, successors, heuristic, |c| *c == end);
    search_result.expect("No path to end").1
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
