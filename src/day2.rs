use aoc_runner_derive::{aoc, aoc_generator};

pub enum Direction {
    Up,
    Down,
    Forward,
}

#[aoc_generator(day2)]
fn steer_inputs(input: &str) -> Vec<(Direction, i32)> {
    input.lines().map(steer_line).collect()
}

fn steer_line(line: &str) -> (Direction, i32) {
    let mut tokens = line.split_ascii_whitespace();
    let dir = match tokens.next().unwrap() {
        "forward" => Direction::Forward,
        "up" => Direction::Up,
        "down" => Direction::Down,
        _ => unreachable!(),
    };
    let amount = tokens.next().unwrap().parse().unwrap();
    (dir, amount)
}

#[aoc(day2, part1)]
pub fn part1(input: &[(Direction, i32)]) -> i64 {
    let (mut pos_x, mut pos_y) = (0i32, 0i32);
    for (direction, amount) in input {
        let amount = *amount;
        match direction {
            Direction::Forward => pos_x += amount,
            Direction::Up => pos_y -= amount,
            Direction::Down => pos_y += amount,
        };
    }
    pos_x as i64 * pos_y as i64
}

#[aoc(day2, part2)]
pub fn part2(input: &[(Direction, i32)]) -> i64 {
    let (mut pos_x, mut pos_y) = (0i32, 0i32);
    let mut aim = 0i32;
    for (direction, amount) in input {
        let amount = *amount;
        match direction {
            Direction::Forward => {
                pos_x += amount;
                pos_y += aim * amount;
            }
            Direction::Up => aim -= amount,
            Direction::Down => aim += amount,
        };
    }
    pos_x as i64 * pos_y as i64
}
