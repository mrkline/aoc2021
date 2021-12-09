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
        self.cells[x + y * self.width]
    }

    fn risk_of_cell(&self, x: usize, y: usize) -> Option<i8> {
        let mut adjacents = [-1i8; 4];

        if x > 0 {
            adjacents[0] = self.cell(x - 1, y);
        }
        if x < self.width - 1 {
            adjacents[1] = self.cell(x + 1, y);
        }
        if y > 0 {
            adjacents[2] = self.cell(x, y - 1);
        }
        if y < self.height - 1 {
            adjacents[3] = self.cell(x, y + 1);
        }

        let center = self.cell(x, y);

        if adjacents
            .into_iter()
            .filter(|a| *a >= 0)
            .all(|a| a > center)
        {
            Some(center + 1)
        } else {
            None
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
            if let Some(r) = input.risk_of_cell(x, y) {
                risk += r as i64;
            }
        }
    }
    risk
}

pub fn basin_size(input: &Heightmap, visited: &mut FixedBitSet, x: usize, y: usize) -> i64 {
    let idx = x + y * input.width;
    if visited.contains(idx) {
        return 0;
    }

    visited.insert(idx);

    let mut size = 1;
    if x > 0 {
        size += basin_size(input, visited, x - 1, y);
    }
    if x < input.width - 1 {
        size += basin_size(input, visited, x + 1, y);
    }
    if y > 0 {
        size += basin_size(input, visited, x, y - 1);
    }
    if y < input.height - 1 {
        size += basin_size(input, visited, x, y + 1);
    }
    size
}

#[aoc(day9, part2)]
pub fn part2(input: &Heightmap) -> i64 {
    let mut visited = FixedBitSet::with_capacity(input.cells.len());

    // Mark all max-height cells as visited.
    for (i, _cell) in input.cells.iter().enumerate().filter(|(_i, c)| **c == 9) {
        visited.insert(i);
    }

    let mut basins = Vec::new();

    for y in 0..input.height {
        for x in 0..input.width {
            let bs = basin_size(input, &mut visited, x, y);
            if bs > 0 {
                basins.push(bs);
            }
        }
    }

    basins.sort_unstable();
    basins.iter().rev().take(3).product()
}
