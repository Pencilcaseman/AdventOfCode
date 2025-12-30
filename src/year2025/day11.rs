use std::collections::{HashMap, HashSet, VecDeque, hash_map::Entry};

use rustc_hash::FxBuildHasher;

type FastHashMap<K, V> = HashMap<K, V, FxBuildHasher>;
type FastHashSet<K> = HashSet<K, FxBuildHasher>;

#[derive(Default, Debug)]
pub struct Node<'a> {
    forward: Vec<&'a str>,
    backward: Vec<&'a str>,
}

type Input<'a> = FastHashMap<&'a str, Node<'a>>;

pub fn parse<'a>(input: &'a str) -> Input<'a> {
    // TODO: Store graph as a matrix: row -> col ==> Node[row] -> Node[col]

    let mut graph = Input::default();

    for line in input.lines() {
        let (from, to_many) = line.split_at(3);

        for to in to_many[2..].split_ascii_whitespace() {
            match graph.entry(from) {
                Entry::Occupied(mut occupied_entry) => {
                    occupied_entry.get_mut().forward.push(to)
                }
                Entry::Vacant(vacant_entry) => {
                    vacant_entry.insert_entry(Node {
                        forward: vec![to],
                        backward: Vec::new(),
                    });
                }
            }

            match graph.entry(to) {
                Entry::Occupied(mut occupied_entry) => {
                    occupied_entry.get_mut().backward.push(from)
                }
                Entry::Vacant(vacant_entry) => {
                    vacant_entry.insert_entry(Node {
                        forward: Vec::new(),
                        backward: vec![from],
                    });
                }
            }
        }
    }

    graph
}

pub fn part1(input: &Input) -> u64 {
    solve(input, "you", "out", None)
}

pub fn part2(input: &Input) -> u64 {
    solve(input, "svr", "fft", Some(&connected_to(input, "fft")))
        * solve(input, "fft", "dac", Some(&connected_to(input, "dac")))
        * solve(input, "dac", "out", Some(&connected_to(input, "out")))
}

pub fn solve(
    input: &Input,
    src: &str,
    dst: &str,
    connected: Option<&FastHashSet<&str>>,
) -> u64 {
    let mut stack = vec![src];
    let mut res = 0;

    while let Some(current) = stack.pop() {
        for &next in &input[current].forward {
            if next == dst {
                res += 1;
                break;
            }

            if connected.is_none_or(|c| c.contains(next)) {
                stack.push(next);
            }
        }
    }

    res
}

fn connected_to<'a>(graph: &'a Input, node: &'a str) -> FastHashSet<&'a str> {
    let mut connected = FastHashSet::<&'a str>::default();

    connected.insert(node);

    let mut stack = vec![node];

    while let Some(next) = stack.pop() {
        // Backward connections are all connected.
        // Forward connections from those backward connections may not be.

        for backward in &graph[next].backward {
            if connected.insert(backward) {
                stack.push(backward);
            }
        }
    }

    connected
}

// Answers for my input:
// Part 1: 494
// Part 2: 296006754704850
