use aoc_runner_derive::{aoc, aoc_generator};

use itertools::Itertools;
use rustc_hash::FxHashMap;

struct DeterministicDie {
    value: i32,
}

impl DeterministicDie {
    fn new() -> Self {
        Self { value: 1 }
    }

    fn roll(&mut self) -> i32 {
        let sum = self.value * 3 + 3;
        self.value += 3;
        sum
    }

    fn num_rolls(&self) -> i32 {
        self.value - 1
    }
}

#[aoc_generator(day21)]
fn parse(input: &str) -> [i8; 2] {
    input
        .lines()
        .map(|l| {
            l.split_ascii_whitespace()
                .rev() // Take the last token.
                .next()
                .unwrap()
                .parse::<i8>()
                .unwrap()
                - 1 // Ordinal to cardinal
        })
        .tuple_windows()
        .map(|(p1, p2)| [p1, p2])
        .next()
        .unwrap()
}

#[aoc(day21, part1)]
fn part1(starting_posits: &[i8; 2]) -> i32 {
    let mut p1 = starting_posits[0] as i32;
    let mut p2 = starting_posits[1] as i32;

    let mut p1_score = 0;
    let mut p2_score = 0;

    let mut dd = DeterministicDie::new();
    loop {
        p1 += dd.roll();
        p1 %= 10;
        p1_score += p1 + 1;

        if p1_score >= 1000 {
            break;
        }

        p2 += dd.roll();
        p2 %= 10;
        p2_score += p2 + 1;

        if p2_score >= 1000 {
            break;
        }
    }

    let loser = std::cmp::min(p1_score, p2_score);
    loser as i32 * dd.num_rolls()
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Turn {
    Player1,
    Player2,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct GameState {
    posits: [i8; 2],
    scores: [i8; 2],
    turn: Turn,
}

fn wins(state: &GameState, memory: &mut FxHashMap<GameState, [i64; 2]>) -> [i64; 2] {
    // Memoization:
    if let Some(scores) = memory.get(state) {
        return *scores;
    }

    // Check for WinRARs
    const WIN: i8 = 21;
    if state.scores[0] >= WIN {
        return [1, 0];
    }
    if state.scores[1] >= WIN {
        return [0, 1];
    }

    let mut total_wins = [0; 2];

    for i in 1..=3 {
        for j in 1..=3 {
            for k in 1..=3 {
                let quantum_roll = i + j + k;
                let player = state.turn as usize;

                let mut new_state = *state;
                new_state.posits[player] += quantum_roll;
                new_state.posits[player] %= 10;
                new_state.scores[player] += new_state.posits[player] + 1;

                new_state.turn = if state.turn == Turn::Player1 {
                    Turn::Player2
                } else {
                    Turn::Player1
                };

                let new_wins = wins(&new_state, memory);
                total_wins[0] += new_wins[0];
                total_wins[1] += new_wins[1];
            }
        }
    }

    memory.insert(*state, total_wins);
    total_wins
}

#[aoc(day21, part2)]
fn part2(starting_posits: &[i8; 2]) -> i64 {
    let starting_state = GameState {
        posits: *starting_posits,
        scores: [0, 0],
        turn: Turn::Player1,
    };
    let total_wins = wins(&starting_state, &mut FxHashMap::default());
    std::cmp::max(total_wins[0], total_wins[1])
}
