#![warn(clippy::pedantic, clippy::nursery)]

use atoi::atoi;

/// Parse the input
///
/// # Panics
///
/// Panics if the input is not in a valid format.
#[must_use]
pub fn parse(input: &str) -> Vec<usize> {
    input
        .as_bytes()
        .split(|b| *b == b'\n')
        .filter(|b| !b.is_empty())
        .map(|row| {
            // Numbers are at most 99, so this is safe
            let mut found = [false; 100];

            let colon = row.iter().position(|b| *b == b':').unwrap() + 1;
            let pipe = row.iter().position(|b| *b == b'|').unwrap();

            row[colon..pipe]
                .split(|b| *b == b' ')
                .filter(|b| !b.is_empty())
                .for_each(|b| {
                    found[atoi::<usize>(b).unwrap()] = true;
                });

            row[pipe + 1..]
                .split(|b| *b == b' ')
                .filter(|b| !b.is_empty())
                .map(|b| usize::from(found[atoi::<usize>(b).unwrap()]))
                .sum()
        })
        .collect()
}

/// Solve part 1 of the question
#[must_use]
pub fn part1(input: &[usize]) -> usize {
    // Equivalent to 1 << (c - 1) but without a subtraction
    input.iter().map(|&c| (1 << c) >> 1).sum()
}

/// Solve part 2 of the question
#[must_use]
pub fn part2(input: &[usize]) -> usize {
    let mut repeats = vec![1; input.len()];

    input.iter().enumerate().for_each(|(i, &n)| {
        (0..n).for_each(|j| {
            repeats[i + j + 1] += repeats[i];
        });
    });

    repeats.iter().sum()
}

// For my input, the correct answers are:
// Part 1: 20117
// Part 2: 13768818
