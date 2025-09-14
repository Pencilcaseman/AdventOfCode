#![warn(clippy::pedantic, clippy::nursery)]

use std::collections::HashSet;

use rayon::prelude::*;
use rustc_hash::FxBuildHasher;

use crate::util::point::{Direction, Point2D};

type FastHashSet<T> = HashSet<T, FxBuildHasher>;
type Input = (usize, usize);
type SeenHashSet = FastHashSet<(Point2D<isize>, Direction)>;

#[allow(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_possible_wrap
)]
#[must_use]
pub fn parse(input: &str) -> Input {
    let mut grid = Vec::with_capacity(128);
    let mut row = Vec::new();
    let mut pos = Point2D::new(0, 0);

    let mut row_idx = 0;
    let mut col_idx = 0;

    for byte in input.bytes() {
        match byte {
            b'\n' => {
                grid.push(row.clone());
                row.clear();

                row_idx += 1;
                col_idx = 0;
            }
            b'^' => {
                pos = Point2D::new(row_idx, col_idx);
                row.push(b'^');
            }
            other => {
                col_idx += 1;
                row.push(other);
            }
        }
    }

    if !row.is_empty() {
        grid.push(row);
    }

    let rows = grid.len();
    let cols = grid[0].len();

    let mut path = Vec::with_capacity(8192);
    let mut dir = Direction::Up;
    let mut tmp_pos = pos + dir;

    while tmp_pos.row >= 0
        && tmp_pos.row < rows as isize
        && tmp_pos.col >= 0
        && tmp_pos.col < cols as isize
    {
        if grid[tmp_pos.row as usize][tmp_pos.col as usize] == b'#' {
            dir = dir.clockwise();
            tmp_pos = pos + dir;
            continue;
        }

        let next = pos + dir;

        if grid[next.row as usize][next.col as usize] == b'.' {
            path.push((pos, dir));
            grid[next.row as usize][next.col as usize] = b'^';
        }

        pos = next;
        tmp_pos = pos + dir;
    }

    let skipper = Skipper::from(&grid);

    let part1 = path.len() + 1;
    let part2 = path
        .par_chunks(64)
        .map(|chunk| {
            let mut seen = SeenHashSet::default();

            let mut count = 0;
            for (pos, dir) in chunk {
                seen.clear();
                if is_loop(&skipper, *pos, (rows, cols), *dir, &mut seen) {
                    count += 1;
                }
            }
            count
        })
        .sum();

    (part1, part2)
}

#[must_use]
pub const fn part1(input: &Input) -> usize {
    input.0
}

#[must_use]
pub const fn part2(input: &Input) -> usize {
    input.1
}

#[allow(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_possible_wrap,
    clippy::match_on_vec_items
)]
#[must_use]
pub fn is_loop(
    skipper: &Skipper,
    mut pos: Point2D<isize>,
    (rows, cols): (usize, usize),
    mut dir: Direction,
    seen: &mut SeenHashSet,
) -> bool {
    let obstacle = pos + dir;

    while pos.row >= 0
        && pos.row < rows as isize
        && pos.col >= 0
        && pos.col < cols as isize
    {
        if !seen.insert((pos, dir)) {
            return true;
        }

        match dir {
            Direction::Up => {
                let target = skipper.up[pos.row as usize][pos.col as usize];

                if pos.col == obstacle.col
                    && pos.row > obstacle.row
                    && obstacle.row >= target
                {
                    pos.row = obstacle.row + 1;
                } else {
                    pos.row = target;
                }
            }
            Direction::Down => {
                let target = skipper.down[pos.row as usize][pos.col as usize];

                if pos.col == obstacle.col
                    && pos.row < obstacle.row
                    && obstacle.row <= target
                {
                    pos.row = obstacle.row - 1;
                } else {
                    pos.row = target;
                }
            }
            Direction::Left => {
                let target = skipper.left[pos.row as usize][pos.col as usize];

                if pos.row == obstacle.row
                    && pos.col > obstacle.col
                    && obstacle.col >= target
                {
                    pos.col = obstacle.col + 1;
                } else {
                    pos.col = target;
                }
            }
            Direction::Right => {
                let target = skipper.right[pos.row as usize][pos.col as usize];

                if pos.row == obstacle.row
                    && pos.col < obstacle.col
                    && obstacle.col <= target
                {
                    pos.col = obstacle.col - 1;
                } else {
                    pos.col = target;
                }
            }
        }

        dir = dir.clockwise();
    }

    false
}

pub struct Skipper {
    up: Vec<Vec<isize>>,
    down: Vec<Vec<isize>>,
    left: Vec<Vec<isize>>,
    right: Vec<Vec<isize>>,
}

impl Skipper {
    #[allow(
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss,
        clippy::cast_possible_wrap,
        clippy::match_on_vec_items
    )]
    #[must_use]
    pub fn from(grid: &[Vec<u8>]) -> Self {
        let mut up = vec![vec![0; grid[0].len()]; grid.len()];
        let mut down = vec![vec![0; grid[0].len()]; grid.len()];
        let mut left = vec![vec![0; grid[0].len()]; grid.len()];
        let mut right = vec![vec![0; grid[0].len()]; grid.len()];

        let rows = grid.len();
        let cols = grid[0].len();

        for col in 0..cols {
            let mut last = -1;

            for row in 0..rows {
                match grid[row][col] {
                    b'#' => {
                        last = row as isize + 1;
                    }
                    _ => {
                        up[row][col] = last;
                    }
                }
            }
        }

        for col in 0..cols {
            let mut last = rows as isize;

            for row in (0..rows).rev() {
                match grid[row][col] {
                    b'#' => {
                        last = row as isize - 1;
                    }
                    _ => {
                        down[row][col] = last;
                    }
                }
            }
        }

        for row in 0..rows {
            let mut last = -1;

            for col in 0..cols {
                match grid[row][col] {
                    b'#' => {
                        last = col as isize + 1;
                    }
                    _ => {
                        left[row][col] = last;
                    }
                }
            }
        }

        for row in 0..rows {
            let mut last = cols as isize;

            for col in (0..cols).rev() {
                match grid[row][col] {
                    b'#' => {
                        last = col as isize - 1;
                    }
                    _ => {
                        right[row][col] = last;
                    }
                }
            }
        }

        Self { up, down, left, right }
    }
}

// For my input, the correct answer is:
// Part 1: 5199
// Part 2: 1915
