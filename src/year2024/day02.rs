#![warn(clippy::pedantic, clippy::nursery)]

use crate::util::parse;

#[must_use]
pub fn parse(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| {
            parse::ParseUnsigned::<u32>::new(line.bytes()).collect::<Vec<_>>()
        })
        .collect()
}

#[must_use]
pub fn part1(input: &[Vec<u32>]) -> usize {
    input
        .iter()
        .filter(|row| {
            (row.iter().zip(row.iter().skip(1)).all(|(&a, &b)| a < b)
                || row.iter().zip(row.iter().skip(1)).all(|(&a, &b)| a > b))
                && row
                    .iter()
                    .zip(row.iter().skip(1))
                    .all(|(&a, &b)| matches!(a.abs_diff(b), 1..4))
        })
        .count()
}

#[must_use]
pub fn part2(input: &[Vec<u32>]) -> u32 {
    // todo!()
    0
}

// For my input, the correct answer is:
// Part 1: 407
// Part 2:
