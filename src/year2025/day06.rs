use itertools::Itertools;

use crate::util::parse::ParseUnsigned;

type Input = (u64, u64);

pub fn parse(input: &str) -> Input {
    let mut lines = input.splitn(5, '\n');
    let a = ParseUnsigned::<u64>::new(lines.next().unwrap().bytes());
    let b = ParseUnsigned::<u64>::new(lines.next().unwrap().bytes());
    let c = ParseUnsigned::<u64>::new(lines.next().unwrap().bytes());
    let d = ParseUnsigned::<u64>::new(lines.next().unwrap().bytes());
    let op = lines.next().unwrap().bytes().filter(|&b| b == b'+' || b == b'*');

    let part1: u64 = a
        .zip(b)
        .zip(c)
        .zip(d)
        .zip(op)
        .map(|((((a, b), c), d), op)| {
            if op == b'+' {
                a + b + c + d
            } else if op == b'*' {
                a * b * c * d
            } else {
                panic!()
            }
        })
        .sum();

    (part1, 0)
}

pub fn part1(input: &Input) -> u64 {
    input.0
}

pub fn part2(input: &Input) -> u64 {
    input.1
}

// Answers for my input:
// Part 1: 6209956042374
// Part 2:
