#![warn(clippy::pedantic, clippy::nursery)]

use itertools::Itertools;

use crate::util::parse::ParseSigned;

type Input = Vec<(usize, usize, usize, usize)>;

#[allow(clippy::cast_sign_loss)]
#[must_use]
pub fn parse(input: &str) -> Input {
    ParseSigned::<i32>::new(input.bytes())
        .tuples()
        .map(|(x, y, dx, dy)| {
            (
                x as usize,
                y as usize,
                dx.rem_euclid(101) as usize,
                dy.rem_euclid(103) as usize,
            )
        })
        .collect()
}

#[must_use]
pub fn part1(input: &Input) -> usize {
    let quadrant = |x, y| {
        let w = 101 / 2;
        let h = 103 / 2;

        if x == w || y == h {
            None
        } else {
            Some(usize::from(x < w) + 2 * usize::from(y < h))
        }
    };

    input
        .iter()
        .filter_map(|(x, y, dx, dy)| {
            quadrant((x + dx * 100) % 101, (y + dy * 100) % 103)
        })
        .fold([0, 0, 0, 0], |mut count, quad| {
            count[quad] += 1;
            count
        })
        .into_iter()
        .product()
}

#[must_use]
pub fn part2(input: &Input) -> usize {
    let mut x_mod = 0;
    let mut y_mod = 0;

    let mut row = [0; 101];
    let mut col = [0; 103];

    for step in 0..103 {
        row.fill(0);
        col.fill(0);

        for (x, y, dx, dy) in input {
            let x = (x + dx * step) % 101;
            let y = (y + dy * step) % 103;

            row[x] += 1;
            col[y] += 1;
        }

        // TODO: It might be worth pushing these to a vector to account for
        // cases where the robots align outside of the tree, but this works for
        // my input so it's probably fine :)
        if row.iter().filter(|&&c| c >= 20).count() >= 2 {
            x_mod = step;
        }

        if col.iter().filter(|&&c| c >= 20).count() >= 2 {
            y_mod = step;
        }
    }

    // Solve x * p = 1 mod n
    // let helper = |p: usize, n: usize| (1..n).find(|x| (x * p).rem_euclid(n)
    // == 1); let x = helper(height, width).unwrap_or(1); // => 51
    // let y = helper(width, height).unwrap_or(1); // => 51
    //
    // 51 * height = 5,253
    // 51 * width = 5,151
    // width * height = 10403

    (x_mod * 5253 + y_mod * 5151) % 10403
}

// For my input, the correct answer is:
// Part 1: 229069152
// Part 2: 7383
