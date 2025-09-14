#![warn(clippy::pedantic, clippy::nursery)]

use std::collections::HashMap;

use rustc_hash::FxBuildHasher;

type FastHashMap<K, V> = HashMap<K, V, FxBuildHasher>;

type InputMap = FastHashMap<u8, Vec<(usize, usize)>>;
type Input = (InputMap, (usize, usize));

#[must_use]
pub fn parse(input: &str) -> Input {
    let mut rows = 0;
    let mut cols = 0;
    let mut map = InputMap::with_capacity_and_hasher(128, FxBuildHasher {});

    for (row, line) in input.lines().enumerate() {
        for (col, byte) in line.bytes().enumerate() {
            if matches!(byte,
                b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9')
            {
                map.entry(byte).or_default().push((row, col));
            }

            cols = col.max(cols);
        }

        rows = row.max(rows);
    }

    (map, (rows + 1, cols + 1))
}

#[must_use]
pub fn part1((map, (rows, cols)): &Input) -> usize {
    let mut seen = ndarray::Array2::zeros((*rows, *cols));

    for nodes in map.values() {
        for (first_row, first_col) in nodes {
            for (second_row, second_col) in nodes {
                if first_row == second_row && first_col == second_col {
                    continue;
                }

                let d1_row = second_row.wrapping_sub(*first_row);
                let d1_col = second_col.wrapping_sub(*first_col);

                let p1_row = first_row.wrapping_sub(d1_row);
                let p1_col = first_col.wrapping_sub(d1_col);

                if p1_row < *rows && p1_col < *cols {
                    seen[(p1_row, p1_col)] = 1;
                }
            }
        }
    }

    seen.sum()
}

#[must_use]
pub fn part2((map, (rows, cols)): &Input) -> usize {
    let mut seen = ndarray::Array2::zeros((*rows, *cols));

    for nodes in map.values() {
        for (first_row, first_col) in nodes {
            for (second_row, second_col) in nodes {
                let mut first_row = *first_row;
                let mut first_col = *first_col;

                if first_row == *second_row && first_col == *second_col {
                    continue;
                }

                let d1_row = second_row.wrapping_sub(first_row);
                let d1_col = second_col.wrapping_sub(first_col);

                while first_row < *rows && first_col < *cols {
                    seen[(first_row, first_col)] = 1;

                    first_row = first_row.wrapping_sub(d1_row);
                    first_col = first_col.wrapping_sub(d1_col);
                }
            }
        }
    }

    seen.sum()
}

// For my input, the correct answer is:
// Part 1: 348
// Part 2: 1221
