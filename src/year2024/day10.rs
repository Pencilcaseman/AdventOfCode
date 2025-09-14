#![warn(clippy::pedantic, clippy::nursery)]

use num_traits::WrappingAdd;

use crate::util::point::{Direction, Point2D};

type Input = ndarray::Array2<u8>;

#[must_use]
pub fn parse(input: &str) -> Input {
    let cols = input.bytes().position(|b| b == b'\n').unwrap_or(1);
    let data = input.bytes().filter(|&b| b != b'\n').collect::<Vec<_>>();
    let rows = data.len() / cols;

    unsafe { Input::from_shape_vec_unchecked((rows, cols), data) }
}

pub fn dfs(
    input: &Input,
    seen: &mut ndarray::Array2<u32>,
    id: u32,
    // pos: (usize, usize),
    pos: Point2D<usize>,
    distinct: bool,
) -> u32 {
    let mut res = 0;

    for new in Point2D::orthogonal().into_iter().map(|d| pos.wrapping_add(&d)) {
        if new.row < input.dim().0
            && new.col < input.dim().1
            && input[pos.tuple()].wrapping_sub(input[new.tuple()]) == 1
            && (!distinct || seen[new.tuple()] != id)
        {
            seen[new.tuple()] = id;

            if input[new.tuple()] == b'0' {
                res += 1;
            } else {
                res += dfs(input, seen, id, new, distinct);
            }
        }
    }

    res
}

#[must_use]
pub fn solve(input: &Input, distinct: bool) -> u32 {
    let mut seen = ndarray::Array2::<u32>::from_elem(input.raw_dim(), u32::MAX);

    input
        .indexed_iter()
        .filter_map(|((row, col), val)| match *val {
            b'9' => Some(dfs(
                input,
                &mut seen,
                u32::try_from(row * input.dim().0 + col).unwrap_or(0),
                Point2D::new(row, col),
                distinct,
            )),
            _ => None,
        })
        .sum()
}

#[must_use]
pub fn part1(input: &Input) -> u32 {
    solve(input, true)
}

#[must_use]
pub fn part2(input: &Input) -> u32 {
    solve(input, false)
}

// For my input, the correct answer is:
// Part 1: 816
// Part 2: 1960
