use aoc_runner_derive::aoc;

/// Returns Ok(unparsed) or Err(expected)
fn match_incomplete_line(input: &str) -> Result<Vec<u8>, u8> {
    let bytes: &[u8] = input.as_bytes();

    let mut chunk_stack: Vec<u8> = Vec::new();

    fn expect(e: u8, c: Option<u8>) -> Result<(), u8> {
        match c {
            Some(c) if c == e => Ok(()),
            Some(_) => Err(e),
            None => panic!("empty chunk stack"),
        }
    }

    for b in bytes {
        match b {
            b'(' | b'[' | b'{' | b'<' => chunk_stack.push(*b),
            b')' => expect(b'(', chunk_stack.pop())?,
            b']' => expect(b'[', chunk_stack.pop())?,
            b'}' => expect(b'{', chunk_stack.pop())?,
            b'>' => expect(b'<', chunk_stack.pop())?,
            wut => panic!("unexpected token {}", wut),
        }
    }

    Ok(chunk_stack)
}

#[aoc(day10, part1)]
pub fn part1(input: &str) -> i64 {
    input
        .lines()
        .map(match_incomplete_line)
        .filter_map(|res| res.err())
        .map(|expected| match expected {
            b'(' => 3,
            b'[' => 57,
            b'{' => 1197,
            b'<' => 25137,
            _ => panic!("wut"),
        })
        .sum()
}

fn closing_score(chunk_stack: &[u8]) -> i64 {
    let mut total = 0;
    for c in chunk_stack.iter().rev() {
        let score = match c {
            b'(' => 1,
            b'[' => 2,
            b'{' => 3,
            b'<' => 4,
            _ => panic!("wut"),
        };
        total *= 5;
        total += score;
    }
    total
}

#[aoc(day10, part2)]
pub fn part2(input: &str) -> i64 {
    let mut line_scores: Vec<i64> = input
        .lines()
        .map(match_incomplete_line)
        .filter_map(|res| res.ok())
        .map(|unclosed| closing_score(&unclosed))
        .collect();

    let len = line_scores.len() / 2;
    let (_, median, _) = line_scores.select_nth_unstable(len);
    *median
}
