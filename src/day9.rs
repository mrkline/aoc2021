use aoc_runner_derive::{aoc, aoc_generator};

use std::fmt::{Debug, Error, Formatter};

use fixedbitset::FixedBitSet;

pub struct Heightmap {
    cells: Vec<i8>,
    width: usize,
    height: usize,
}

impl Heightmap {
    fn cell(&self, x: usize, y: usize) -> i8 {
        if x < self.width && y < self.height {
            self.cells[x + y * self.width]
        } else {
            -1
        }
    }

    fn low_point_cell(&self, x: usize, y: usize) -> Option<i8> {
        let mut adjacents = [-1i8; 4];

        adjacents[0] = self.cell(x - 1, y);
        adjacents[1] = self.cell(x + 1, y);
        adjacents[2] = self.cell(x, y - 1);
        adjacents[3] = self.cell(x, y + 1);

        let center = self.cell(x, y);

        if adjacents
            .into_iter()
            .filter(|a| *a >= 0)
            .all(|a| a > center)
        {
            Some(center)
        } else {
            None
        }
    }

    fn risk_of_cell(&self, x: usize, y: usize) -> i8 {
        if let Some(cell) = self.low_point_cell(x, y) {
            cell + 1
        } else {
            0
        }
    }
}

impl Debug for Heightmap {
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

#[aoc_generator(day9)]
pub fn parse_heightmap(input: &str) -> Heightmap {
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

    Heightmap {
        cells,
        width,
        height,
    }
}

#[aoc(day9, part1)]
pub fn part1(input: &Heightmap) -> i64 {
    // println!("{:?}", input);
    let mut risk: i64 = 0;

    for y in 0..input.height {
        for x in 0..input.width {
            risk += input.risk_of_cell(x, y) as i64;
        }
    }
    risk
}

pub fn basin_size(input: &Heightmap, visited: &mut FixedBitSet, x: usize, y: usize) -> i64 {
    let idx = x + y * input.width;
    if x >= input.width || y >= input.height || visited.contains(idx) {
        return 0;
    }

    visited.insert(idx);

    1 + basin_size(input, visited, x - 1, y)
        + basin_size(input, visited, x + 1, y)
        + basin_size(input, visited, x, y - 1)
        + basin_size(input, visited, x, y + 1)
}

#[aoc(day9, part2)]
pub fn part2(input: &Heightmap) -> i64 {
    let mut low_points = FixedBitSet::with_capacity(input.cells.len());
    for y in 0..input.height {
        for x in 0..input.width {
            if input.low_point_cell(x, y).is_some() {
                low_points.insert(x + y * input.width);
            }
        }
    }

    let mut visited = FixedBitSet::with_capacity(input.cells.len());

    // Mark all max-height cells as visited.
    for (i, _cell) in input.cells.iter().enumerate().filter(|(_i, c)| **c == 9) {
        visited.insert(i);
    }

    let mut basins = Vec::new();

    for idx in low_points.ones() {
        let y = idx / input.width;
        let x = idx % input.width;
        basins.push(basin_size(input, &mut visited, x, y));
    }

    basins.sort_unstable();
    basins.iter().rev().take(3).product()
}
