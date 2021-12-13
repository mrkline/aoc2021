use aoc_runner_derive::{aoc, aoc_generator};

use fixedbitset::FixedBitSet;

use std::fmt::{Debug, Error, Formatter};

#[derive(Debug, Copy, Clone)]
pub enum Fold {
    X(u16),
    Y(u16),
}

#[derive(Default, Clone)]
pub struct Transparency {
    dots: FixedBitSet,
    width: usize,
    height: usize,
}

impl Transparency {
    fn get(&self, x: usize, y: usize) -> bool {
        self.dots.contains(x + y * self.width)
    }

    fn set(&mut self, x: usize, y: usize) {
        self.dots.insert(x + y * self.width)
    }

    fn fold_up(&self, on: usize) -> Self {
        let height = on;
        let width = self.width;
        let dots = FixedBitSet::with_capacity(height * width);

        let mut folded = Self {
            dots,
            width,
            height,
        };

        // Copy the top half over
        for top_dot in self.dots.ones() {
            if top_dot >= folded.dots.len() {
                break;
            } else {
                folded.dots.insert(top_dot);
            }
        }

        // Apply the bottom half, mirrored, to the top.
        let mut mirrored_y = height - 1;
        for y in (on + 1)..self.height {
            for x in 0..width {
                if self.get(x, y) {
                    folded.set(x, mirrored_y);
                }
            }
            mirrored_y -= 1;
        }

        folded
    }

    fn fold_left(&self, on: usize) -> Self {
        let height = self.height;
        let width = on;
        let dots = FixedBitSet::with_capacity(height * width);

        let mut folded = Self {
            dots,
            width,
            height,
        };

        // Copy the left half over
        for y in 0..height {
            for x in 0..width {
                if self.get(x, y) {
                    folded.set(x, y);
                }
            }
        }

        // Apply the right half, mirrored, to the left
        for y in 0..height {
            let mut mirrored_x = width - 1;
            for x in (on + 1)..self.width {
                if self.get(x, y) {
                    folded.set(mirrored_x, y);
                }
                mirrored_x -= 1;
            }
        }

        folded
    }

    fn fold(&self, f: Fold) -> Self {
        match f {
            Fold::X(on) => self.fold_left(on as usize),
            Fold::Y(on) => self.fold_up(on as usize),
        }
    }
}

impl Debug for Transparency {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        for y in 0..self.height {
            for x in 0..self.width {
                write!(f, "{}", if self.get(x, y) { '#' } else { '.' })?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn parse_points<'a>(input: &'a str) -> (Vec<(u16, u16)>, std::str::Lines<'a>) {
    let mut points = Vec::new();

    let mut lines = input.lines();
    loop {
        let line = lines.next().expect("input ended early");
        if line.is_empty() {
            break;
        }

        let mut comma_split = line.split(',');
        let x = comma_split.next().expect("no x").parse().unwrap();
        let y = comma_split.next().expect("no y").parse().unwrap();
        points.push((x, y));
    }
    (points, lines)
}

fn points_to_transparency(points: &[(u16, u16)]) -> Transparency {
    let mut max_x: usize = 0;
    let mut max_y: usize = 0;

    for p in points {
        let (x, y) = (p.0 as usize, p.1 as usize);
        if x > max_x {
            max_x = x;
        }
        if y > max_y {
            max_y = y;
        }
    }

    let width = max_x + 1;
    let height = max_y + 1;
    let dots = FixedBitSet::with_capacity(width * height);

    let mut dotted = Transparency {
        dots,
        width,
        height,
    };

    for p in points {
        let (x, y) = (p.0 as usize, p.1 as usize);
        dotted.set(x, y);
    }

    dotted
}

fn parse_folds<'a>(lines: std::str::Lines<'a>) -> Vec<Fold> {
    lines
        .map(|line| {
            let mut fold_tokens = line["fold along ".len()..].split('=');
            let axis = fold_tokens.next().unwrap();
            let val = fold_tokens.next().unwrap().parse().unwrap();

            match axis {
                "x" => Fold::X(val),
                "y" => Fold::Y(val),
                what => panic!("{} ain't no axis I ever heard of!", what),
            }
        })
        .collect()
}

#[aoc_generator(day13)]
pub fn parse(input: &str) -> (Transparency, Vec<Fold>) {
    let (points, remaining_lines) = parse_points(input);
    let transparency = points_to_transparency(&points);
    let folds = parse_folds(remaining_lines);

    (transparency, folds)
}

#[aoc(day13, part1)]
pub fn part1((transparency, folds): &(Transparency, Vec<Fold>)) -> usize {
    transparency.fold(folds[0]).dots.ones().count()
}

#[aoc(day13, part2)]
pub fn part2((transparency, folds): &(Transparency, Vec<Fold>)) -> String {
    let folded_message = folds.iter().fold(transparency.clone(), |t, f| t.fold(*f));
    format!("\n{:?}", folded_message)
}
