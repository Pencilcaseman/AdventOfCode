#![warn(clippy::pedantic, clippy::nursery)]

type Input = (u32, u32);

#[must_use]
pub const fn parse(_input: &str) -> Input {
    (0, 0)
}

#[must_use]
pub const fn part1(input: &Input) -> u32 {
    input.0
}

#[must_use]
pub const fn part2(input: &Input) -> u32 {
    input.1
}

// For my input, the correct answer is:
// Part 1:
// Part 2:
