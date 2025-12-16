use itertools::Itertools;

use crate::util::{parse::ParseUnsigned, point::Direction};

type Input = Vec<(u64, u64)>;

pub fn parse(input: &str) -> Input {
    ParseUnsigned::<u64>::new(input.bytes()).tuples().collect()
}

pub fn part1(input: &Input) -> u64 {
    // dumb version

    input
        .iter()
        .enumerate()
        .flat_map(|(i, p0)| input.iter().skip(i + 1).map(|p1| area(p0, p1)))
        .max()
        .unwrap()
}

pub fn part2(input: &Input) -> u64 {
    let len = input.len();

    0
}

fn area(p0: &(u64, u64), p1: &(u64, u64)) -> u64 {
    (p0.0.abs_diff(p1.0) + 1) * (p0.1.abs_diff(p1.1) + 1)
}

// Answers for my input:
// Part 1: 4744899849
// Part 2:
