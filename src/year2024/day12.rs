#![warn(clippy::pedantic, clippy::nursery)]

use std::collections::{HashSet, VecDeque};

use rustc_hash::FxBuildHasher;

type FastHashSet<V> = HashSet<V, FxBuildHasher>;

type Input<'a> = Vec<&'a [u8]>;

#[must_use]
pub fn parse(input: &str) -> Input<'_> {
    input.as_bytes().split(|&b| b == b'\n').collect()
}

// PERF: Slow
fn to_count_grid(input: &Input) -> Vec<Vec<u8>> {
    let rows = input.len();
    let cols = input[0].len();
    let mut res = vec![vec![0; cols]; rows];

    for row in 0..rows {
        for col in 0..cols {
            let current = input[row][col];

            let up = if row == 0 {
                1
            } else {
                u8::from(input[row - 1][col] != current)
            };

            let down = if row == rows - 1 {
                1
            } else {
                u8::from(input[row + 1][col] != current)
            };

            let left = if col == 0 {
                1
            } else {
                u8::from(input[row][col - 1] != current)
            };

            let right = if col == cols - 1 {
                1
            } else {
                u8::from(input[row][col + 1] != current)
            };

            res[row][col] = up + down + left + right;
        }
    }

    res
}

fn right_the_first_version_didnt_work(input: &Input) -> u32 {
    let rows = input.len();
    let cols = input[0].len();

    let mut alien = VecDeque::from(vec![(0, 0)]);
    let mut open = VecDeque::new();

    // let mut seen = FastHashSet::default();
    let mut seen = vec![vec![false; cols]; rows];

    let mut res = 0;

    while let Some(plot) = alien.pop_front() {
        if !seen[plot.0][plot.1] {
            open.push_back(plot);

            let mut area = 0;
            let mut perimeter = 0;

            while let Some((row, col)) = open.pop_front() {
                // if !seen.insert((row, col)) {
                //     continue;
                // }

                if seen[row][col] {
                    continue;
                }

                seen[row][col] = true;

                let current = input[row][col];

                area += 1;

                // Up
                if row == 0 {
                    perimeter += 1;
                } else if input[row - 1][col] == current {
                    open.push_back((row - 1, col));
                } else {
                    perimeter += 1;
                    alien.push_back((row - 1, col));
                }

                // Down
                if row == rows - 1 {
                    perimeter += 1;
                } else if input[row + 1][col] == current {
                    open.push_back((row + 1, col));
                } else {
                    perimeter += 1;
                    alien.push_back((row + 1, col));
                }

                // Left
                if col == 0 {
                    perimeter += 1;
                } else if input[row][col - 1] == current {
                    open.push_back((row, col - 1));
                } else {
                    perimeter += 1;
                    alien.push_back((row, col - 1));
                }

                // Right
                if col == cols - 1 {
                    perimeter += 1;
                } else if input[row][col + 1] == current {
                    open.push_back((row, col + 1));
                } else {
                    perimeter += 1;
                    alien.push_back((row, col + 1));
                }
            }

            res += area * perimeter;
        }
    }

    res
}

#[must_use]
pub fn part1(input: &Input) -> u32 {
    right_the_first_version_didnt_work(input)

    // let mut mapping = FastHashMap::default();
    // let grid = to_count_grid(input);
    //
    // println!("Grid:\n{grid:?}");
    //
    // let rows = grid.len();
    // let cols = grid[0].len();
    //
    // for row in 0..rows {
    //     for col in 0..cols {
    //         let current = input[row][col];
    //         let val = mapping.entry(current).or_insert((0u32, 0u32));
    //
    //         val.0 += 1;
    //         val.1 += u32::from(grid[row][col]);
    //     }
    // }
    //
    // println!("Mapping: {mapping:?}");
    //
    // mapping.into_values().map(|v| v.0 * v.1).sum()
}

#[must_use]
pub fn part2(input: &Input) -> u32 {
    1
}

// For my input, the correct answer is:
// Part 1:
// Part 2:
