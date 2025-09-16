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

#[allow(clippy::cast_sign_loss)]
#[must_use]
pub fn solve_part2(input: &Input, width: i32, height: i32) -> i32 {
    let mut x_mod = 0;
    let mut y_mod = 0;

    let mut row = vec![0; width as usize];
    let mut col = vec![0; height as usize];

    for step in 0..width.max(height) {
        row.fill(0);
        col.fill(0);

        for (x, y, dx, dy) in input {
            let x = (x + dx * step).rem_euclid(width) as usize;
            let y = (y + dy * step).rem_euclid(height) as usize;

            row[x] += 1;
            col[y] += 1;
        }

        if row.iter().filter(|&&c| c >= 20).count() >= 2 {
            x_mod = step;
        }

        if col.iter().filter(|&&c| c >= 20).count() >= 2 {
            y_mod = step;
        }
    }

    // Solve x * p = 1 mod n
    let helper = |p: i32, n: i32| (1..n).find(|x| (x * p).rem_euclid(n) == 1);

    let x = helper(height, width).unwrap_or(1);
    let y = helper(width, height).unwrap_or(1);

    (x * x_mod * height + y * y_mod * width).rem_euclid(width * height)
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
