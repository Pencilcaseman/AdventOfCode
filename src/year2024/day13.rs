//! This question is really about solving a system of linear equations in 2D.
//!
//! For example, consider the first example given in the problem definition:
//!
//! ```none
//! Button A: X+94, Y+34
//! Button B: X+22, Y+67
//! Prize: X=8400, Y=5400
//! ```
//!
//! We can ignore the 'fewest tokens' part for now and convert this question
//! into a pair of equations:
//!
//! a * 94 + b * 22 = 8400
//! a * 34 + b * 67 = 67
//!
//! Rewriting this as linear algebra problem, we get:
//!
//! [[94 34]  [[a]  = [[8400]
//!  [22 67]]  [b]]    [5400]]
//!
//! =>
//!                 -1
//! [[a]  = [[94 34]   [[8400]
//!  [b]]    [22 67]]   [5400]]
//!
//! =>
//!
//! [[a]  = ________1________ [[67 -34]  [[8400]  = [[80]
//!  [b]]   94 * 67 - 22 * 34  [-22 94]]  [5400]]    [40]]
//!
//! So, with 80xA and 40xB we get to the prize.
//!
//! By checking if the division is exact we can select only integer multiples
//! of the button presses.
//!
//! It is worth noting that there are two possible cases if the determinant of
//! the matrix is zero:
//!
//! 1. Both buttons move the claw in opposite directions
//! 2. Both buttons move the claw in the same direction
//!
//! In the first case, there are an infinite number of solutions. Fortunately,
//! all the inputs are positive and hence this case is not possible.
//!
//! In the second case, there are possibly multiple solutions and we have to
//! find the most efficient one. We know that one button moves the claw some
//! scalar multiple of the other. If this ratio is an integer, there is a fairly
//! simple solution to the problem. If, however, the ratio is not an integer,
//! the problem becomes a little bit more difficult to solve and would probably
//! require the use of explicit fractions as opposed to floating point
//! arithmetic. Fortunately, it seems like none of the inputs ever have a
//! discriminant of zero, so we can ignore this case as well.

#![warn(clippy::pedantic, clippy::nursery)]

use itertools::Itertools;

use crate::util::parse::ParseUnsigned;

type Input = (i64, i64);

const OFFSET: i64 = 10_000_000_000_000;

const fn solve(ax: i64, ay: i64, bx: i64, by: i64, px: i64, py: i64) -> i64 {
    let inv_sf = ax * by - ay * bx;
    let alpha = by * px - bx * py;
    let beta = ax * py - ay * px;

    if alpha % inv_sf != 0 || beta % inv_sf != 0 {
        0
    } else {
        (alpha * 3 + beta) / inv_sf
    }
}

/// # Panics
#[must_use]
pub fn parse(input: &str) -> Input {
    ParseUnsigned::<u32, _>::new(input.bytes())
        .tuples()
        .map(|(ax, ay, bx, by, px, py)| {
            (
                i64::from(ax),
                i64::from(ay),
                i64::from(bx),
                i64::from(by),
                i64::from(px),
                i64::from(py),
            )
        })
        .fold((0, 0), |(part1, part2), (ax, ay, bx, by, px, py)| {
            let p1 = solve(ax, ay, bx, by, px, py);
            let p2 = solve(ax, ay, bx, by, px + OFFSET, py + OFFSET);
            (part1 + p1, part2 + p2)
        })
}

#[must_use]
pub const fn part1(input: &Input) -> i64 {
    input.0
}

#[must_use]
pub const fn part2(input: &Input) -> i64 {
    input.1
}

// For my input, the correct answer is:
// Part 1: 29877
// Part 2: 99423413811305
