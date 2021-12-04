use aoc_runner_derive::aoc;

use fixedbitset::FixedBitSet;

use std::fmt::{Debug, Error, Formatter};
use std::str::Lines;

#[derive(Default)]
struct Board {
    spaces: Vec<u8>,
    marks: FixedBitSet,
}

impl Board {
    fn parse(input: &mut Lines<'_>) -> Self {
        let spaces = input
            .take(5) // Call Paul Desmond
            .map(|line| {
                // Map each line into a vec of numbers
                line.split_ascii_whitespace()
                    .map(|tok| tok.parse().unwrap())
                    .collect()
            })
            .fold(vec![], |mut acc, mut l| {
                // fold them together
                acc.append(&mut l);
                acc
            });
        assert_eq!(spaces.len(), 25);
        let marks = FixedBitSet::with_capacity(25);
        Board { spaces, marks }
    }

    fn space(&self, x: usize, y: usize) -> u8 {
        self.spaces[y * 5 + x]
    }

    fn is_marked(&self, x: usize, y: usize) -> bool {
        self.marks.contains(y * 5 + x)
    }

    fn mark(&mut self, num: u8) {
        for (i, space) in self.spaces.iter().enumerate() {
            if *space == num {
                self.marks.insert(i);
            }
        }
    }

    fn won(&self) -> bool {
        for y in 0..5 {
            if (0..5).all(|x| self.is_marked(x, y)) {
                return true;
            }
        }
        for x in 0..5 {
            if (0..5).all(|y| self.is_marked(x, y)) {
                return true;
            }
        }
        false
    }

    fn score(&self, last_num: u8) -> u32 {
        let last_num = last_num as u32;
        let unmarked: u32 = self
            .spaces
            .iter()
            .enumerate()
            .filter_map(|(i, s)| if !self.marks.contains(i) {
                Some(*s as u32)
            } else {
                None
            })
            .sum();

        last_num * unmarked
    }
}

impl Debug for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        for y in 0..5 {
            writeln!(
                f,
                "[{:2} {:2} {:2} {:2} {:2}]",
                self.space(0, y),
                self.space(1, y),
                self.space(2, y),
                self.space(3, y),
                self.space(4, y)
            )?;
        }
        for y in 0..5 {
            writeln!(
                f,
                "{{{} {} {} {} {}}}",
                self.is_marked(0, y) as u8,
                self.is_marked(1, y) as u8,
                self.is_marked(2, y) as u8,
                self.is_marked(3, y) as u8,
                self.is_marked(4, y) as u8,
            )?;
        }
        Ok(())
    }
}

fn parse_bingo_numbers(num_line: &str) -> Vec<u8> {
    num_line.split(',').map(|n| n.parse().unwrap()).collect()
}

fn parse_boards(lines: &mut Lines<'_>) -> Vec<Board> {
    let mut boards = Vec::new();

    while let Some(empty_line) = lines.next() {
        assert!(empty_line.is_empty()); // Empty line between boards
        boards.push(Board::parse(lines));
    }

    boards
}

#[aoc(day4, part1)]
pub fn part1(input: &str) -> u32 {
    let mut lines = input.lines();
    let numbers = parse_bingo_numbers(lines.next().unwrap());
    let mut boards = parse_boards(&mut lines);

    for num in numbers {
        for board in &mut boards {
            board.mark(num);
            if board.won() {
                // println!("{}! Bingo!\n{:?}", num, board);
                return board.score(num);
            }
        }
    }

    unreachable!("No winning board!");
}

#[aoc(day4, part2)]
pub fn part2(input: &str) -> u32 {
    let mut lines = input.lines();
    let numbers = parse_bingo_numbers(lines.next().unwrap());
    let mut boards = parse_boards(&mut lines);

    let mut last_num = 0;
    for num in numbers {
        last_num = num;
        for board in &mut boards {
            board.mark(num);
        }
        if boards.len() > 1 {
            // If we've got multiple boards left, throw out ones that won.
            boards.retain(|b| !b.won());
        } else if boards[0].won() {
            // Otherwise just keep playing the last board until it wins
            break;
        }
    }

    assert_eq!(boards.len(), 1);
    let last_board = &boards[0];

    assert!(last_board.won());
    // println!("Last board:\n{:?}", last_board);

    last_board.score(last_num)
}
