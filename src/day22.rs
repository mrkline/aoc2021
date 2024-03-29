#![allow(unused)]

use aoc_runner_derive::{aoc, aoc_generator};

use bitvec::prelude::*;
use itertools::Itertools;

use nalgebra as na;

type Point = na::geometry::Point3<i32>;

#[derive(Debug)]
struct BoundingBox {
    min: Point,
    max: Point,
}

impl BoundingBox {
    fn dimensions(&self) -> Point {
        (self.max - self.min).into()
    }

    fn volume(&self) -> i64 {
        let d = self.dimensions();
        d.x as i64 * d.y as i64 * d.z as i64
    }

    fn contains_point(&self, p: &Point) -> bool {
        (self.min.x <= p.x && p.x <= self.max.x)
            && (self.min.y <= p.y && p.y <= self.max.y)
            && (self.min.z <= p.z && p.z <= self.max.z)
    }

    fn contains_box(&self, b: &BoundingBox) -> bool {
        self.contains_point(&b.min) && self.contains_point(&b.max)
    }

    fn overlaps_box(&self, b: &BoundingBox) -> bool {
        let x1min = self.min.x;
        let y1min = self.min.y;
        let z1min = self.min.z;

        let x1max = self.max.x;
        let y1max = self.max.y;
        let z1max = self.max.z;

        let x2min = b.min.x;
        let y2min = b.min.y;
        let z2min = b.min.z;

        let x2max = b.max.x;
        let y2max = b.max.y;
        let z2max = b.max.z;

        // https://stackoverflow.com/a/20925869
        (x1min <= x2max && x2min <= x1max)
            && (y1min <= y2max && y2min <= y1max)
            && (z1min <= z2max && z2min <= z1max)
    }

    fn expand(&self, b: &BoundingBox) -> Self {
        let xmin = std::cmp::min(self.min.x, b.min.x);
        let ymin = std::cmp::min(self.min.y, b.min.y);
        let zmin = std::cmp::min(self.min.z, b.min.z);

        let xmax = std::cmp::max(self.max.x, b.max.x);
        let ymax = std::cmp::max(self.max.y, b.max.y);
        let zmax = std::cmp::max(self.max.z, b.max.z);

        let min = Point::new(xmin, ymin, zmin);
        let max = Point::new(xmax, ymax, zmax);

        Self { min, max }
    }
}

#[derive(Debug)]
struct Instruction {
    bounds: BoundingBox,
    on: bool,
}

#[aoc_generator(day22)]
fn parse_instructions(input: &str) -> Vec<Instruction> {
    input.lines().map(parse_instruction).collect()
}

fn parse_instruction(line: &str) -> Instruction {
    let mut tokens = line.split_ascii_whitespace();
    let on = match tokens.next().unwrap() {
        "on" => true,
        "off" => false,
        wut => panic!("{} isn't on or off", wut),
    };
    let cube = tokens.next().unwrap();
    let mut axes = cube.split(',');
    let x_axis = axes.next().unwrap();
    let y_axis = axes.next().unwrap();
    let z_axis = axes.next().unwrap();

    assert!(x_axis.starts_with("x="));
    assert!(y_axis.starts_with("y="));
    assert!(z_axis.starts_with("z="));

    let (left, right) = parse_axis(x_axis);
    let (top, bottom) = parse_axis(y_axis);
    let (front, back) = parse_axis(z_axis);

    assert!(left <= right);
    assert!(top <= bottom);
    assert!(front <= back);

    let min = Point::new(left, top, front);
    let max = Point::new(right, bottom, back);
    let bounds = BoundingBox { min, max };

    Instruction { bounds, on }
}

fn parse_axis(dim: &str) -> (i32, i32) {
    dim[2..]
        .split("..")
        .map(|n| n.parse().unwrap())
        .tuple_windows()
        .next()
        .unwrap()
}

#[aoc(day22, part1)]
fn part1(instructions: &[Instruction]) -> usize {
    let mut voxels: BitVec<Lsb0, usize> = BitVec::repeat(false, 101*101*101);

    let mut touch = |x, y, z, on| {
        let index = ((z + 50) * 101 * 101) + ((y + 50) * 101) + (x + 50);
        voxels.set(index as usize, on);
    };

    let interested_area = BoundingBox {
        min: Point::new(-50, -50, -50),
        max: Point::new(50, 50, 50),
    };

    for inst in instructions {
        let b = &inst.bounds;
        if !interested_area.overlaps_box(b) {
            continue
        }

        for z in -50..=50 {
            for y in -50..=50 {
                for x in -50..=50 {
                    let p = Point::new(x, y, z);
                    if b.contains_point(&p) {
                        touch(x, y, z, inst.on);
                    }
                }
            }
        }
    }

    voxels.count_ones()
}
