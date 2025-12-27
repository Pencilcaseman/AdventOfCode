use std::collections::{HashMap, HashSet, VecDeque, hash_map::Entry};

type Input<'a> = HashMap<&'a str, Vec<&'a str>>;

pub fn parse<'a>(input: &'a str) -> Input<'a> {
    let mut graph = Input::default();

    for line in input.lines() {
        let (from, to) = line.split_at(3);

        match graph.entry(from) {
            Entry::Occupied(mut occupied_entry) => occupied_entry
                .get_mut()
                .extend(to[2..].split_ascii_whitespace()),
            Entry::Vacant(vacant_entry) => {
                vacant_entry
                    .insert_entry(to[2..].split_ascii_whitespace().collect());
            }
        }
    }

    graph
}

pub fn part1(input: &Input) -> u32 {
    let mut stack = vec!["you"];
    let mut res = 0;

    while let Some(current) = stack.pop() {
        for next in &input[current] {
            if *next != "out" {
                stack.push(next);
            } else {
                res += 1;
            }
        }
    }

    res
}

pub fn part2(input: &Input) -> u32 {
    let mut stack = vec!["svr"];
    let mut res = 0;

    while let Some(current) = stack.pop() {
        for next in &input[current] {
            if *next != "out" {
                stack.push(next);
            } else {
                res += 1;
            }
        }
    }

    res
}

// Answers for my input:
// Part 1: 494
// Part 2:
