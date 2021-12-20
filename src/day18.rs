use aoc_runner_derive::{aoc, aoc_generator};

use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
enum SnailElement {
    Num(u8),
    Pair(SnailPair),
}

impl SnailElement {
    fn left_leaf(&mut self) -> &mut u8 {
        match self {
            SnailElement::Num(i) => i,
            SnailElement::Pair(p) => p.left.left_leaf(),
        }
    }

    fn right_leaf(&mut self) -> &mut u8 {
        match self {
            SnailElement::Num(i) => i,
            SnailElement::Pair(p) => p.right.right_leaf(),
        }
    }
}

impl fmt::Display for SnailElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SnailElement::Num(n) => write!(f, "{}", n),
            SnailElement::Pair(p) => write!(f, "{}", p),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct SnailPair {
    left: Box<SnailElement>,
    right: Box<SnailElement>,
}

impl fmt::Display for SnailPair {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{},{}]", *self.left, *self.right)
    }
}

fn add(left: SnailElement, right: SnailElement) -> SnailPair {
    let left = Box::new(left);
    let right = Box::new(right);
    let mut pair = SnailPair { left, right };

    while reduce(&mut pair) {}
    pair
}

fn reduce(pair: &mut SnailPair) -> bool {
    explode(pair) || split(pair)
}

fn split(pair: &mut SnailPair) -> bool {
    // Bail early from the left side so that we always take the leftmost split.
    match &mut *pair.left {
        SnailElement::Num(_) => {
            if split_num(&mut *pair.left) {
                return true;
            }
        }
        SnailElement::Pair(p) => {
            if split(p) {
                return true;
            }
        }
    };

    match &mut *pair.right {
        SnailElement::Num(_) => split_num(&mut *pair.right),
        SnailElement::Pair(p) => split(p),
    }
}

fn split_num(elem: &mut SnailElement) -> bool {
    match elem {
        SnailElement::Num(n) => {
            if *n >= 10 {
                let round_down = *n / 2;
                let round_up = (*n + 1) / 2;
                let p = SnailPair {
                    left: Box::new(SnailElement::Num(round_down)),
                    right: Box::new(SnailElement::Num(round_up)),
                };
                *elem = SnailElement::Pair(p);
                true
            } else {
                false
            }
        }
        SnailElement::Pair(_) => unreachable!(),
    }
}

fn explode(pair: &mut SnailPair) -> bool {
    explode_rec(pair, 0) != Splode::None
}

#[derive(Debug, PartialEq, Eq)]
enum Splode {
    None,
    Boom,
    PropagateLeft(u8),
    PropagateRight(u8),
    PropagateBoth((u8, u8)),
}

fn explode_rec(pair: &mut SnailPair, depth: usize) -> Splode {
    match (&*pair.left, &*pair.right) {
        (SnailElement::Num(l), SnailElement::Num(r)) if depth >= 4 => {
            return Splode::PropagateBoth((*l, *r))
        }
        _ => (),
    };

    if let SnailElement::Pair(p) = &mut *pair.left {
        match explode_rec(p, depth + 1) {
            Splode::None => {}
            // Bail early on all explosions so we only consider the leftmost
            Splode::Boom => {
                return Splode::Boom;
            }
            Splode::PropagateLeft(l) => {
                return Splode::PropagateLeft(l);
            }
            Splode::PropagateRight(r) => {
                *pair.right.left_leaf() += r;
                return Splode::Boom;
            }
            Splode::PropagateBoth((l, r)) => {
                *pair.right.left_leaf() += r;
                *pair.left = SnailElement::Num(0);
                return Splode::PropagateLeft(l);
            }
        }
    }

    if let SnailElement::Pair(p) = &mut *pair.right {
        match explode_rec(p, depth + 1) {
            Splode::None => {}
            Splode::Boom => {
                return Splode::Boom;
            }
            Splode::PropagateRight(r) => {
                return Splode::PropagateRight(r);
            }
            Splode::PropagateLeft(l) => {
                *pair.left.right_leaf() += l;
                return Splode::Boom;
            }
            Splode::PropagateBoth((l, r)) => {
                *pair.left.right_leaf() += l;
                *pair.right = SnailElement::Num(0);
                return Splode::PropagateRight(r);
            }
        }
    }

    Splode::None
}

#[aoc_generator(day18)]
fn parse_lines(input: &str) -> SnailPair {
    input
        .lines()
        .map(parse_line)
        .reduce(|acc, n| {
            let left = SnailElement::Pair(acc);
            let right = SnailElement::Pair(n);
            add(left, right)
        })
        .expect("Odd number of lines")
}

fn parse_line(input: &str) -> SnailPair {
    let mut bytes = input.as_bytes();
    let pair = parse_pair(&mut bytes);
    assert_eq!(bytes.len(), 0);
    pair
}

fn parse_pair(bytes: &mut &[u8]) -> SnailPair {
    assert_eq!(bytes[0], b'[');
    *bytes = &bytes[1..];

    let left = Box::new(parse_element(bytes));
    assert_eq!(bytes[0], b',');
    *bytes = &bytes[1..];
    let right = Box::new(parse_element(bytes));

    assert_eq!(bytes[0], b']');
    *bytes = &bytes[1..];

    SnailPair { left, right }
}

fn parse_element(bytes: &mut &[u8]) -> SnailElement {
    if (b'0'..=b'9').contains(&bytes[0]) {
        let val = bytes[0] - b'0';
        *bytes = &bytes[1..];
        SnailElement::Num(val)
    } else {
        SnailElement::Pair(parse_pair(bytes))
    }
}

fn pair_magnitude(pair: &SnailPair) -> i64 {
    let l = element_magnitude(&*pair.left);
    let r = element_magnitude(&*pair.right);
    l * 3 + r * 2
}

fn element_magnitude(elem: &SnailElement) -> i64 {
    match elem {
        SnailElement::Num(n) => *n as i64,
        SnailElement::Pair(p) => pair_magnitude(p)
    }
}

#[aoc(day18, part1)]
fn part1(pair: &SnailPair) -> i64 {
    pair_magnitude(pair)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn smoke_line_parse() {
        assert_eq!("[[1,2],3]", parse_line("[[1,2],3]").to_string());
        assert_eq!("[9,[8,7]]", parse_line("[9,[8,7]]").to_string());
        assert_eq!(
            "[[[[1,3],[5,3]],[[1,3],[8,7]]],[[[4,9],[6,9]],[[8,2],[7,3]]]]",
            parse_line("[[[[1,3],[5,3]],[[1,3],[8,7]]],[[[4,9],[6,9]],[[8,2],[7,3]]]]").to_string()
        );
    }

    #[test]
    fn basic_line_fold() {
        let input = "[1,1]\n[2,2]\n[3,3]\n[4,4]";
        assert_eq!(
            parse_lines(&input).to_string(),
            "[[[[1,1],[2,2]],[3,3]],[4,4]]"
        );
    }

    #[test]
    fn add_lines() {
        let mut input = "[1,1]\n[2,2]\n[3,3]\n[4,4]\n[5,5]";
        assert_eq!(
            parse_lines(&input).to_string(),
            "[[[[3,0],[5,3]],[4,4]],[5,5]]"
        );

        input = "[1,1]\n[2,2]\n[3,3]\n[4,4]\n[5,5]\n[6,6]";
        assert_eq!(
            parse_lines(&input).to_string(),
            "[[[[5,0],[7,4]],[5,5]],[6,6]]"
        );

        // Big examples:
        input = r"[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]
[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]
[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]
[7,[5,[[3,8],[1,4]]]]
[[2,[2,2]],[8,[8,1]]]
[2,9]
[1,[[[9,3],9],[[9,0],[0,7]]]]
[[[5,[7,4]],7],1]
[[[[4,2],2],6],[8,7]]";
        assert_eq!(
            parse_lines(&input).to_string(),
            "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]"
        );

        input = r"[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]";
        assert_eq!(
            parse_lines(&input).to_string(),
            "[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]"
        );
    }

    #[test]
    fn explosions() {
        let mut num = parse_line("[[[[[9,8],1],2],3],4]");
        assert!(explode(&mut num));
        assert_eq!(num.to_string(), "[[[[0,9],2],3],4]");

        num = parse_line("[7,[6,[5,[4,[3,2]]]]]");
        assert!(explode(&mut num));
        assert_eq!(num.to_string(), "[7,[6,[5,[7,0]]]]");

        num = parse_line("[[6,[5,[4,[3,2]]]],1]");
        assert!(explode(&mut num));
        assert_eq!(num.to_string(), "[[6,[5,[7,0]]],3]");

        num = parse_line("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]");
        assert!(explode(&mut num));
        assert_eq!(num.to_string(), "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]");

        num = parse_line("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]");
        assert!(explode(&mut num));
        assert_eq!(num.to_string(), "[[3,[2,[8,0]]],[9,[5,[7,0]]]]");
    }

    #[test]
    fn addition() {
        let left = SnailElement::Pair(parse_line("[[[[4,3],4],4],[7,[[8,4],9]]]"));
        let right = SnailElement::Pair(parse_line("[1,1]"));

        let mut joined = SnailPair {
            left: Box::new(left.clone()),
            right: Box::new(right.clone()),
        };
        assert_eq!(joined.to_string(), "[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]");

        // Explode once
        assert!(reduce(&mut joined));
        assert_eq!(joined.to_string(), "[[[[0,7],4],[7,[[8,4],9]]],[1,1]]");

        // Explode again
        assert!(reduce(&mut joined));
        assert_eq!(joined.to_string(), "[[[[0,7],4],[15,[0,13]]],[1,1]]");

        // Split
        assert!(reduce(&mut joined));
        assert_eq!(joined.to_string(), "[[[[0,7],4],[[7,8],[0,13]]],[1,1]]");

        // Split again
        assert!(reduce(&mut joined));
        assert_eq!(joined.to_string(), "[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]");

        // Explode
        assert!(reduce(&mut joined));
        assert_eq!(joined.to_string(), "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");

        // Nothing left
        let same = joined.clone();
        assert!(!reduce(&mut joined));
        assert_eq!(joined, same);

        // All together now:
        let sum = add(left, right);
        assert_eq!(sum.to_string(), "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");
    }
}
