#![warn(clippy::pedantic, clippy::nursery)]

use hashbrown::HashMap;

use crate::util::parse;

#[must_use]
pub fn parse(input: &str) -> (Vec<u32>, Vec<u32>) {
    parse::ParseUnsigned::<u32>::new(input.bytes())
        .array_chunks::<2>()
        .map(|c| (c[0], c[1]))
        .unzip()
}

#[must_use]
pub fn part1(input: &(Vec<u32>, Vec<u32>)) -> u32 {
    let mut left = input.0.clone();
    let mut right = input.1.clone();

    left.sort_unstable();
    right.sort_unstable();

    left.into_iter().zip(right).map(|(l, r)| l.abs_diff(r)).sum()
}

#[must_use]
pub fn part2(input: &(Vec<u32>, Vec<u32>)) -> u32 {
    let mut map = HashMap::<u32, u32>::with_capacity(1024);
    input.0.iter().for_each(|n| *map.entry(*n).or_default() += 1);
    input.1.iter().filter_map(|n| map.get(n).map(|c| c * n)).sum()
}

// For my input, the correct answer is:
// Part 1: 1830467
// Part 2: 26674158
