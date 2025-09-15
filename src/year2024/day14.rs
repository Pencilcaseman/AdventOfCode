#![warn(clippy::pedantic, clippy::nursery)]

use itertools::Itertools;

use crate::util::parse::ParseSigned;

type Input = Vec<(i32, i32, i32, i32)>;

#[must_use]
pub fn parse(input: &str) -> Input {
    ParseSigned::<i32>::new(input.bytes()).tuples().collect()
}

#[must_use]
pub fn solve_part1(input: &Input, width: i32, height: i32, steps: i32) -> i32 {
    let quadrant = |x, y| {
        let w = width / 2;
        let h = height / 2;

        if x == w || y == h {
            None
        } else {
            Some(usize::from(x < w) + 2 * usize::from(y < h))
        }
    };

    input
        .iter()
        .filter_map(|(x, y, dx, dy)| {
            quadrant(
                (x + dx * steps).rem_euclid(width),
                (y + dy * steps).rem_euclid(height),
            )
        })
        .fold([0, 0, 0, 0], |mut count, quad| {
            count[quad] += 1;
            count
        })
        .into_iter()
        .product()
}

#[must_use]
pub fn solve_part2(input: &Input, width: i32, height: i32) -> i32 {
    let mut grid =
        ndarray::Array2::from_elem((height as usize, width as usize), false);

    let mut steps = 0;

    loop {
        steps += 1;
        let mut valid = true;
        grid.fill(false);

        for (col, row) in input.iter().map(|(x, y, dx, dy)| {
            (
                (x + dx * steps).rem_euclid(width),
                (y + dy * steps).rem_euclid(height),
            )
        }) {
            unsafe {
                if *grid.uget((row as usize, col as usize)) {
                    valid = false;
                    break;
                }

                *grid.uget_mut((row as usize, col as usize)) = true;
            }
        }

        if !valid {
            continue;
        }

        break steps;
    }
}

#[must_use]
pub fn part1(input: &Input) -> i32 {
    solve_part1(input, 101, 103, 100)
}

#[must_use]
pub fn part2(input: &Input) -> i32 {
    solve_part2(input, 101, 103)
}

// For my input, the correct answer is:
// Part 1: 229069152
// Part 2: 7383
