use aoc_runner_derive::aoc;

use std::fmt::{Debug, Error, Formatter};

use bitvec::prelude::*;

struct Image {
    pixels: BitVec,
    width: usize,
    height: usize,

    /// Is the infinite space around us lit?
    lit_expanse: bool,
}

impl Image {
    /// Get a pixel - out-of-bounds values are furnished by the expanse
    fn pixel(&self, x: isize, y: isize) -> usize {
        let x = x as usize;
        let y = y as usize;

        (if x >= self.width || y >= self.height {
            self.lit_expanse
        } else {
            self.pixels[x + y * self.width]
        }) as usize
    }

    // Get the LUT index for the current pixel
    fn kernel_lut_index(&self, x: isize, y: isize) -> usize {
        // Overflow & underflow is fine, pixel() handles it above.
        self.pixel(x - 1, y - 1) << 8
            | self.pixel(x, y - 1) << 7
            | self.pixel(x + 1, y - 1) << 6
            | self.pixel(x - 1, y) << 5
            | self.pixel(x, y) << 4
            | self.pixel(x + 1, y) << 3
            | self.pixel(x - 1, y + 1) << 2
            | self.pixel(x, y + 1) << 1
            | self.pixel(x + 1, y + 1)
    }
}

impl Debug for Image {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        for y in 0..self.height {
            for x in 0..self.width {
                let c = if self.pixels[x + y * self.width] {
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
    lut: BitVec,
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

#[inline]
fn parse_byte(b: u8) -> bool {
    match b {
        b'#' => true,
        b'.' => false,
        wut => panic!("{} isn't # or .", wut),
    }
}

fn parse_lut(line: &str) -> BitVec {
    let bytes = line.as_bytes();
    assert_eq!(bytes.len(), 512);

    bytes.iter().copied().map(parse_byte).collect()
}

fn parse_image(lines: &mut std::str::Lines<'_>) -> Image {
    let mut pixels = BitVec::new();
    let mut width: usize = 0;
    let mut height: usize = 0;

    for line in lines {
        let bytes = line.as_bytes();
        width = bytes.len();

        for b in bytes.iter().copied().map(parse_byte) {
            pixels.push(b);
        }

        height += 1;
    }
    assert_eq!(width, height);

    Image {
        pixels,
        width,
        height,
        lit_expanse: false,
    }
}

fn enhance(image: &Image, lut: &BitSlice) -> Image {
    // All kernels in the expanse will be all zeroes or all ones.
    let lit_expanse: bool = if image.lit_expanse {
        *lut.last().unwrap()
    } else {
        *lut.first().unwrap()
    };

    // Since our kernel is 3x3, the neighboring 2 pixels in every direction
    // will be influenced by the previous image.
    let width = image.width + 2 * 2;
    let height = image.height + 2 * 2;
    let mut pixels = BitVec::with_capacity(width * height);

    for y in -2isize..(image.height as isize + 2) {
        for x in -2isize..(image.width as isize + 2) {
            let index = image.kernel_lut_index(x, y);
            pixels.push(lut[index]);
        }
    }

    Image {
        pixels,
        width,
        height,
        lit_expanse,
    }
}

#[aoc(day20, part1)]
fn part1(input: &str) -> usize {
    let Input { lut, mut image } = parse(input);
    image = enhance(&image, &lut);
    image = enhance(&image, &lut);
    image.pixels.count_ones()
}

#[aoc(day20, part2)]
fn part2(input: &str) -> usize {
    let Input { lut, mut image } = parse(input);
    for _ in 0..50 {
        image = enhance(&image, &lut);
    }
    image.pixels.count_ones()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn kernel_smoke() {
        let img = Image {
            width: 3,
            height: 3,
            pixels: bitvec![0, 0, 0, 1, 0, 0, 0, 1, 0],
            lit_expanse: false,
        };

        assert_eq!(img.kernel_lut_index(1, 1), 34);
        assert_eq!(img.kernel_lut_index(2, 1), 0b100);
        assert_eq!(img.kernel_lut_index(3, 1), 0);
        assert_eq!(img.kernel_lut_index(1, 0), 0b100);
        assert_eq!(img.kernel_lut_index(0, -1), 0);
    }

    #[test]
    fn sample_input() {
        let input = r"..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###";

        let Input { lut, mut image } = parse(input);
        println!("{:?}", image);
        image = enhance(&image, &lut);
        println!("{:?}", image);
        image = enhance(&image, &lut);
        println!("{:?}", image);

        assert_eq!(image.pixels.count_ones(), 35);

        for _ in 2..50 {
            image = enhance(&image, &lut);
        }
        assert_eq!(image.pixels.count_ones(), 3351);
    }
}
