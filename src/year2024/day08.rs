#![warn(clippy::pedantic, clippy::nursery)]

use std::collections::HashMap as StdHashMap;

use fxhash::FxBuildHasher;

type FastHashMap<K, V> = StdHashMap<K, V, FxBuildHasher>;

type InputMap = FastHashMap<u8, Vec<(usize, usize)>>;
type Input = (InputMap, (usize, usize));

struct Grid<T> {
    data: Vec<T>,
    cols: usize,
}

impl<T: Default + Clone> Grid<T> {
    fn new(rows: usize, cols: usize) -> Self {
        Self { data: vec![T::default(); rows * cols], cols }
    }
}

impl<T> std::ops::Index<(usize, usize)> for Grid<T> {
    type Output = T;

    fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
        &self.data[row * self.cols + col]
    }
}

impl<T> std::ops::IndexMut<(usize, usize)> for Grid<T> {
    fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut Self::Output {
        &mut self.data[row * self.cols + col]
    }
}

impl<T> IntoIterator for Grid<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

#[must_use]
pub fn parse(input: &str) -> Input {
    let mut rows = 0;
    let mut cols = 0;
    let mut map =
        InputMap::with_capacity_and_hasher(128, FxBuildHasher::default());

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
    let mut seen = Grid::new(*rows, *cols);

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

    seen.into_iter().sum()
}

#[must_use]
pub fn part2((map, (rows, cols)): &Input) -> usize {
    let mut seen = Grid::new(*rows, *cols);

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

    seen.into_iter().sum()
}

// For my input, the correct answer is:
// Part 1: 348
// Part 2: 1221
