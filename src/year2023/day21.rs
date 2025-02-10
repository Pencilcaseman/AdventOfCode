#![warn(clippy::pedantic, clippy::nursery)]

use std::collections::VecDeque;

#[must_use]
pub fn parse(input: &str) -> (usize, usize) {
    let table: Vec<Vec<u8>> =
        input.lines().map(|line| line.as_bytes().to_vec()).collect::<Vec<_>>();
    let origin = (65, 65);
    let (inner_even, inner_odd, outer_even, outer_odd) =
        walker(&table, &[origin], 131);

    let part1 = inner_even;

    let (inner_even_corners, ..) =
        walker(&table, &[(0, 0), (0, 130), (130, 0), (130, 130)], 65);

    let even_total = inner_even + outer_even;
    let odd_total = inner_odd + outer_odd;

    let n = 202_300;

    let even = n * n * even_total;
    let odd = (n + 1) * (n + 1) * odd_total;
    let even_edges = n * inner_even_corners;
    let odd_edges = (n + 1) * outer_odd;

    let part2 = even + odd + even_edges - odd_edges;

    (part1, part2)
}

#[must_use]
pub const fn part1(input: &(usize, usize)) -> usize {
    input.0
}

#[must_use]
pub const fn part2(input: &(usize, usize)) -> usize {
    input.1
}

#[must_use]
pub fn walker(
    grid: &[Vec<u8>],
    start_points: &[(usize, usize)],
    max_steps: usize,
) -> (usize, usize, usize, usize) {
    let mut grid = grid.to_vec();

    let mut todo = VecDeque::new();
    for (r, c) in start_points {
        grid[*r][*c] = b'#'; // Mark as visited
        todo.push_back(((*r, *c), 0));
    }

    let mut inner_even = 0;
    let mut inner_odd = 0;
    let mut outer_even = 0;
    let mut outer_odd = 0;

    while let Some(((row, col), steps)) = todo.pop_front() {
        if steps % 2 == 0 {
            if steps <= 65 {
                inner_even += 1;
            } else {
                outer_even += 1;
            }
        } else if steps <= 65 {
            inner_odd += 1;
        } else {
            outer_odd += 1;
        }

        for &(r, c) in &[
            (row.wrapping_sub(1), col),
            (row.wrapping_add(1), col),
            (row, col.wrapping_sub(1)),
            (row, col.wrapping_add(1)),
        ] {
            if steps < max_steps
                && r < grid.len()
                && c < grid[0].len()
                && grid[r][c] != b'#'
            {
                grid[r][c] = b'#';
                todo.push_back(((r, c), steps + 1));
            }
        }
    }

    (inner_even, inner_odd, outer_even, outer_odd)
}

// For my input, the correct answer is:
// Part 1: 3591
// Part 2: 598044246091826
