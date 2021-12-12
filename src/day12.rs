use aoc_runner_derive::aoc;

use rustc_hash::{FxHashMap, FxHashSet};

// A graph maps a node name to a set of its neighbors
pub type Graph<'a> = FxHashMap<&'a str, FxHashSet<&'a str>>;

pub fn parse_graph(input: &str) -> Graph {
    let mut graph = Graph::default();

    for line in input.lines() {
        let mut tokens = line.split('-');
        let left = tokens.next().unwrap();
        let right = tokens.next().unwrap();

        // Don't loop back to start or come back from end
        let left_node = graph.entry(left).or_default();
        if right != "start" && left != "end" {
            left_node.insert(right);
        }

        let right_node = graph.entry(right).or_default();
        if left != "start" && right != "end" {
            right_node.insert(left);
        }
    }

    graph
}

fn paths_single_small_visit<'a>(
    graph: &Graph<'a>,
    current: &'a str,
    mut past: FxHashSet<&'a str>,
) -> usize {
    // We win!
    if current == "end" {
        return 1;
    }

    if current.chars().all(|c| c.is_lowercase()) {
        past.insert(current);
    }

    let mut all_paths = 0;

    for to in &graph[current] {
        // Don't visit small caves twice.
        let small_cave = to.chars().all(|c| c.is_lowercase());
        if small_cave && past.contains(to) {
            continue;
        }

        all_paths += paths_single_small_visit(graph, to, past.clone());
    }

    all_paths
}

fn paths_one_double_small<'a>(
    graph: &Graph<'a>,
    current: &'a str,
    mut past: FxHashSet<&'a str>,
    visited_small_twice: bool,
) -> usize {
    // We win!
    if current == "end" {
        return 1;
    }

    if current.chars().all(|c| c.is_lowercase()) {
        past.insert(current);
    }

    let mut all_paths = 0;

    for to in &graph[current] {
        // Visit _one_ small cave twice.
        let small_cave = to.chars().all(|c| c.is_lowercase());
        if small_cave && past.contains(to) {
            if visited_small_twice {
                continue;
            } else {
                all_paths += paths_one_double_small(graph, to, past.clone(), true);
            }
        } else {
            all_paths += paths_one_double_small(graph, to, past.clone(), visited_small_twice);
        }
    }

    all_paths
}

#[aoc(day12, part1)]
pub fn part1(input: &str) -> usize {
    let graph = parse_graph(input);

    paths_single_small_visit(&graph, "start", FxHashSet::default())
}

#[aoc(day12, part2)]
pub fn part2(input: &str) -> usize {
    let graph = parse_graph(input);

    paths_one_double_small(&graph, "start", FxHashSet::default(), false)
}
