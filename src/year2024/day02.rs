#![warn(clippy::pedantic, clippy::nursery)]

use crate::util::parse;

#[must_use]
pub fn parse(input: &str) -> (i32, i32) {
    let mut part1 = 0;
    let mut part2 = 0;
    let mut nums = Vec::new();

    for line in input.lines() {
        nums.extend(parse::ParseSigned::<i32>::new(line.bytes()));
        let (p1, p2) = solve_row(&nums);
        nums.clear();

        part1 += p1;
        part2 += p2;
    }

    (part1, part2)
}

#[must_use]
pub const fn part1(input: &(i32, i32)) -> i32 {
    input.0
}

#[must_use]
pub const fn part2(input: &(i32, i32)) -> i32 {
    input.1
}

const fn delta((&a, &b): (&i32, &i32)) -> i32 {
    let diff = b - a;

    if diff.abs() < 4 { diff.signum() } else { 0 }
}

fn solve_row(row: &[i32]) -> (i32, i32) {
    let score: i32 = row.iter().zip(row.iter().skip(1)).map(delta).sum();

    // Strictly monotonoically increasing or decreasing
    if score.abs() == i32::try_from(row.len()).unwrap() - 1 {
        return (1, 1);
    }

    // Not monotonoically increasing or decreasing, so we try removing each
    // element and see how that affects the score
    for i in 0..row.len() {
        let mut score = score;

        // This does the same thing, but I think is a bit more clear.
        // if i == 0 {
        //     // Remove the first element from the row
        //     score -= delta((&row[0], &row[1]));
        // } else if i == row.len() - 1 {
        //     // Remove the last element from the row
        //     score -= delta((&row[i - 1], &row[i]));
        // } else {
        //     // Remove the middle element from the row
        //     score -= delta((&row[i - 1], &row[i]));
        //     score -= delta((&row[i], &row[i + 1]));
        //     score += delta((&row[i - 1], &row[i + 1]));
        // }

        if i > 0 {
            score -= delta((&row[i - 1], &row[i]));
        }
        if i < row.len() - 1 {
            score -= delta((&row[i], &row[i + 1]));
        }
        if i > 0 && i < row.len() - 1 {
            score += delta((&row[i - 1], &row[i + 1]));
        }

        if score.abs() == i32::try_from(row.len()).unwrap() - 2 {
            return (0, 1);
        }
    }

    (0, 0)
}

// For my input, the correct answer is:
// Part 1: 407
// Part 2: 459
