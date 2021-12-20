use aoc_runner_derive::aoc;

use std::fmt::{Debug, Error, Formatter};

use fixedbitset::FixedBitSet;

struct Image {
    pixels: FixedBitSet,
    width: usize,
    height: usize,

    /// Is the infinite space around us lit?
    lit_expanse: bool,
}

impl Debug for Image {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        for y in 0..self.height {
            for x in 0..self.width {
                let c = if self.pixels.contains(x + y * self.width) {
                    '#'
                } else {
                    '.'
                };
                write!(f, "{}", c)?;
            }
            writeln!(f)?;
        }
        if self.lit_expanse {
            writeln!(f, "Expanse is lit")?;
        } else {
            writeln!(f, "Expanse is dark")?;
        }
        Ok(())
    }
}

struct Input {
    lut: FixedBitSet,
    image: Image,
}

impl Debug for Input {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        writeln!(f, "{:b}", self.lut)?;
        writeln!(f, "{:?}", self.image)
    }
}

fn parse(input: &str) -> Input {
    let mut lines = input.lines();
    let lut = parse_lut(lines.next().unwrap());
    assert_eq!(lines.next(), Some(""));
    let image = parse_image(&mut lines);

    Input { lut, image }
}

fn parse_lut(line: &str) -> FixedBitSet {
    let bytes = line.as_bytes();
    assert_eq!(bytes.len(), 512);

    let mut lut = FixedBitSet::with_capacity(512);
    for (i, b) in bytes.iter().enumerate() {
        match b {
            b'#' => lut.insert(i),
            b'.' => {}
            wut => panic!("{} isn't # or .", wut),
        };
    }
    lut
}

fn parse_image(lines: &mut std::str::Lines<'_>) -> Image {
    let mut pixels = None;
    let mut width: usize = 0;
    let mut height: usize = 0;

    for line in lines {
        let bytes = line.as_bytes();
        width = bytes.len();

        if pixels.is_none() {
            pixels = Some(FixedBitSet::with_capacity(width * width));
        }

        for (i, b) in bytes.iter().enumerate() {
            match b {
                b'#' => pixels.as_mut().unwrap().insert(i + width * height),
                b'.' => {}
                wut => panic!("{} isn't # or .", wut),
            };
        }

        height += 1;
    }
    assert_eq!(width, height);

    Image {
        pixels: pixels.unwrap(),
        width,
        height,
        lit_expanse: false,
    }
}

#[aoc(day20, part1)]
fn part1(input: &str) -> i64 {
    let Input { lut, mut image } = parse(input);
    42
}
