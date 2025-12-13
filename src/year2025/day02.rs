use itertools::Itertools;

use crate::util::{
    integer::{num_length, pow10},
    parse::ParseUnsigned,
};

type Input = (u64, u64);

const DOUBLE_COUNT_REMOVAL: [i64; 16] =
    [0, 0, 1, 1, 0, 1, -1, 1, 0, 0, -1, 1, 0, 1, -1, -1];

pub fn parse(input: &str) -> Input {
    let mut p1 = 0;
    let mut p2 = 0;

    for vals in ParseUnsigned::new(input.bytes()).tuples::<(u64, u64)>() {
        let (a1, a2) = single_sum(vals.0 - 1);
        let (b1, b2) = single_sum(vals.1);

        p1 += b1 - a1;
        p2 += b2 - a2;
    }

    (p1, p2)
}

pub fn part1(input: &Input) -> u64 {
    input.0
}

pub fn part2(input: &Input) -> u64 {
    input.1
}

fn p(r: u8, q: u8) -> u64 {
    (pow10(r * q) - 1) / (pow10(q) - 1)
}

fn t(a: u64, r: u8, q: u8) -> u64 {
    (a / p(r, q)).min(pow10(q) - 1)
}

fn sum_between(a: u64, b: u64) -> u64 {
    (a + b) * (b - a + 1) / 2
}

fn single_sum_r(n: u64, r: u8) -> u64 {
    let mut sum = 0;
    let mut q = 1;

    // r * q < 20 only required so result fits in 64 bits. It is not required
    // for the math itself to work
    while r * q < 20 && pow10(q - 1) * p(r, q) <= n {
        let a = pow10(q - 1);
        let b = t(n, r, q);
        sum += p(r, q) * sum_between(a, b);
        q += 1;
    }

    sum
}

fn single_sum(n: u64) -> (u64, u64) {
    let p1 = single_sum_r(n, 2);
    let p2 = p1
        + (3..=num_length(n))
            .map(|r| {
                let sign = DOUBLE_COUNT_REMOVAL[r as usize];
                let val = single_sum_r(n, r) as i64;
                sign * val
            })
            .sum::<i64>() as u64;

    (p1, p2)
}

// Answers for my input
// Part 1: 23560874270
// Part 2: 44143124633
