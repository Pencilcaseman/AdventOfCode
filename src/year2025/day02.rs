use crate::util::{
    integer::{num_length, pow10},
    parse::ParseUnsigned,
};

type Input = Vec<(u64, u64)>;

const DOUBLE_COUNT_REMOVAL: [i8; 16] =
    [0, 0, 1, 1, 0, 1, -1, 1, 0, 0, -1, 1, 0, 1, -1, -1];

pub fn parse(input: &str) -> Input {
    ParseUnsigned::new(input.bytes())
        .array_chunks::<2>()
        .map(|vals| (vals[0], vals[1]))
        .collect()
}

pub fn part1(input: &Input) -> u64 {
    let mut sum = 0;

    for &(a, b) in input.iter() {
        let s1 = single_sum_r(a - 1, 2);
        let s2 = single_sum_r(b, 2);
        sum += s2 - s1;
    }

    sum
}

pub fn part2(input: &Input) -> u64 {
    let mut sum = 0;

    for &(a, b) in input.iter() {
        let s1 = single_sum(a - 1);
        let s2 = single_sum(b);
        sum += s2 - s1;
    }

    sum
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

fn single_sum(n: u64) -> u64 {
    (2..(num_length(n) + 1))
        .map(|r| {
            (DOUBLE_COUNT_REMOVAL[r as usize] as i64)
                * (single_sum_r(n, r)) as i64
        })
        .sum::<i64>() as u64
}

// Answers for my input
// Part 1: 23560874270
// Part 2: 44143124633
