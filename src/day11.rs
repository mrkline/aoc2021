use aoc_runner_derive::aoc;

use std::fmt::{Debug, Error, Formatter};

use fixedbitset::FixedBitSet;

pub struct Octopi {
    cells: Vec<i8>,
    width: usize,
    height: usize,
}

impl Octopi {
    fn increment_all(&mut self) {
        for cell in &mut self.cells {
            *cell += 1;
        }
    }

    fn propagate_flashes(&mut self) {
        let mut to_prop = FixedBitSet::with_capacity(self.cells.len());

        for (idx, cell) in self.cells.iter().enumerate() {
            if *cell > 9 {
                to_prop.insert(idx);
            }
        }

        for idx in to_prop.ones() {
            let y = idx / self.width;
            let x = idx % self.width;
            self.propagate_flash(x, y);
        }
    }

    fn propagate_flash(&mut self, x: usize, y: usize) {
        self.illuminate(x - 1, y - 1);
        self.illuminate(x, y - 1);
        self.illuminate(x + 1, y - 1);
        self.illuminate(x - 1, y);
        self.illuminate(x, y);
        self.illuminate(x + 1, y);
        self.illuminate(x - 1, y + 1);
        self.illuminate(x, y + 1);
        self.illuminate(x + 1, y + 1);
    }

    fn illuminate(&mut self, x: usize, y: usize) {
        if x >= self.width || y >= self.height {
            return;
        }
        let cell = &mut self.cells[x + y * self.width];
        *cell += 1;
        if *cell == 10 {
            self.propagate_flash(x, y);
        }
    }

    fn reset_flashes(&mut self) -> i64 {
        let mut flashes = 0;

        for cell in &mut self.cells {
            if *cell > 9 {
                flashes += 1;
                *cell = 0;
            }
        }

        flashes
    }
}

impl Debug for Octopi {
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

pub fn parse_octos(input: &str) -> Octopi {
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

    Octopi {
        cells,
        width,
        height,
    }
}

#[aoc(day11, part1)]
pub fn part1(input: &str) -> i64 {
    let mut octos = parse_octos(input);
    let mut flashes = 0;

    for _i in 0..100 {
        octos.increment_all();
        octos.propagate_flashes();
        flashes += octos.reset_flashes();
    }

    flashes
}

#[aoc(day11, part2)]
pub fn part2(input: &str) -> i64 {
    let mut octos = parse_octos(input);
    let mut steps = 0;

    while octos.cells.iter().any(|c| *c != 0) {
        octos.increment_all();
        octos.propagate_flashes();
        octos.reset_flashes();
        steps += 1;
    }

    steps
}
