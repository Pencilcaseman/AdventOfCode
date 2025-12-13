//! # Gift Shop
//!
//! We can be very clever with this problem and solve it mathematically:
//!
//! Let $p(r, q) = \frac{10^{rq} - 1}{10^q - 1}$ be a repeat pattern generator
//! where $r$ is the number of repeats and $q$ is the number of digits to
//! repeat.
//!
//! For example, $p(3, 2) = 10101$ is a pattern for repeating three two-digit
//! numbers: $10101 * 15 = 151515$.
//!
//! We need a way to find the largest value $n$ such that $n \cdot p(r, q) <= a$
//! and $n < 10^q$. Here, $n$ is the largest possible value we can use in the
//! pattern generator defined previously ($p(r, q)$).
//!
//! Let $t(a, r, q) = \min{\floor{\frac{a}{p(r, q)}}, 10^q - 1}$.
//!
//! Now, we need to sum over every valid number of segments, every valid segment
//! length and every valid integer below some number $a$.
//!
//! Doing this the naive way results in many values being double-counted. We can
//! use the Mobius function $\mu(n)$ to efficiently remove any double-counted
//! solutions.
//!
//! We now have:
//!
//! $
//! s(a) =
//!     \sum_{n = 2}^{\mathrm{digits}(a)}
//!         \mu(r) \sum_{q \in Q_a}
//!             p(r, q) \sum_{i = 10^{q - 1}}^{t(a, r, q)} i
//! $
//! where $Q_a = \{ q : 10^{q - 1} \cdot p(r, q) <= a \}$.
//!
//! Simplifying, we are left with:
//!
//! $
//! s(a) =
//!     \sum_{n = 2}^{\mathrm{digits}(a)}
//!         \mu(r) \sum_{q \in Q_a}
//!             p(r, q)(10^{q - 1} + t(a, r, q))(t(a, r, q) - 10^{q - 1} + 1)
//! $
//!
//! Finally, we can solve for the number of solutions within a given range
//! $(a, b)$ with $s(b) - s(a - 1)$.
//!
//! The final solution for part two is found by summing the individual results
//! of each range in the input.
//!
//! For part one, we can require that $r = 2$.

use itertools::Itertools;

use crate::util::{
    integer::{num_length, pow10},
    parse::ParseUnsigned,
};

type Input = (u64, u64);

const DOUBLE_COUNT_REMOVAL: [i8; 12] = [0, 0, 1, 1, 0, 1, -1, 1, 0, 0, -1, 1];

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

fn double_sum_between(a: u64, b: u64) -> u64 {
    (a + b) * (b - a + 1)
}

fn single_sum_r(n: u64, r: u8) -> u64 {
    let mut sum = 0;
    let mut q = 1;

    // r * q < 20 only required so the result fits in 64 bits. It is not
    // required for the math itself to work
    while r * q < 20 && pow10(q - 1) * p(r, q) <= n {
        let a = pow10(q - 1);
        let b = t(n, r, q);
        sum += p(r, q) * double_sum_between(a, b);
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
                val * sign as i64
            })
            .sum::<i64>() as u64;

    (p1 >> 1, p2 >> 1)
}

// Answers for my input
// Part 1: 23560874270
// Part 2: 44143124633
