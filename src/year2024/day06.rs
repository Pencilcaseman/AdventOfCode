#![warn(clippy::pedantic, clippy::nursery)]

use std::collections::HashSet;

use fxhash::FxBuildHasher;
use rayon::prelude::*;

type FastHashSet<T> = HashSet<T, FxBuildHasher>;
type Input = (usize, usize);
type SeenHashSet = FastHashSet<((isize, isize), Direction)>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    const fn rotate(self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }
}

impl From<Direction> for (isize, isize) {
    fn from(dir: Direction) -> Self {
        match dir {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
        }
    }
}

impl From<(isize, isize)> for Direction {
    fn from(dir: (isize, isize)) -> Self {
        match dir {
            (-1, 0) => Self::Up,
            (1, 0) => Self::Down,
            (0, -1) => Self::Left,
            (0, 1) => Self::Right,
            _ => unreachable!(),
        }
    }
}

impl std::ops::Add<Direction> for (isize, isize) {
    type Output = (isize, isize);

    fn add(self, dir: Direction) -> Self::Output {
        match dir {
            Direction::Up => (self.0 - 1, self.1),
            Direction::Down => (self.0 + 1, self.1),
            Direction::Left => (self.0, self.1 - 1),
            Direction::Right => (self.0, self.1 + 1),
        }
    }
}

impl std::ops::AddAssign<Direction> for (isize, isize) {
    fn add_assign(&mut self, dir: Direction) {
        *self = *self + dir;
    }
}

impl std::ops::Sub<Direction> for (isize, isize) {
    type Output = (isize, isize);

    fn sub(self, dir: Direction) -> Self::Output {
        match dir {
            Direction::Up => (self.0 + 1, self.1),
            Direction::Down => (self.0 - 1, self.1),
            Direction::Left => (self.0, self.1 + 1),
            Direction::Right => (self.0, self.1 - 1),
        }
    }
}

impl std::ops::SubAssign<Direction> for (isize, isize) {
    fn sub_assign(&mut self, dir: Direction) {
        *self = *self - dir;
    }
}

#[allow(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_possible_wrap
)]
#[must_use]
pub fn parse(input: &str) -> Input {
    let mut grid = Vec::with_capacity(128);
    let mut row = Vec::new();
    let mut pos = (0, 0);

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
                pos = (row_idx, col_idx);
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

    while tmp_pos.0 >= 0
        && tmp_pos.0 < rows as isize
        && tmp_pos.1 >= 0
        && tmp_pos.1 < cols as isize
    {
        if grid[tmp_pos.0 as usize][tmp_pos.1 as usize] == b'#' {
            dir = dir.rotate();
            tmp_pos = pos + dir;
            continue;
        }

        let next = pos + dir;

        if grid[next.0 as usize][next.1 as usize] == b'.' {
            path.push((pos, dir));
            grid[next.0 as usize][next.1 as usize] = b'^';
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
    mut pos: (isize, isize),
    (rows, cols): (usize, usize),
    mut dir: Direction,
    seen: &mut SeenHashSet,
) -> bool {
    let obstacle = pos + dir;

    while pos.0 >= 0
        && pos.0 < rows as isize
        && pos.1 >= 0
        && pos.1 < cols as isize
    {
        if !seen.insert((pos, dir)) {
            return true;
        }

        match dir {
            Direction::Up => {
                let target = skipper.up[pos.0 as usize][pos.1 as usize];

                if pos.1 == obstacle.1
                    && pos.0 > obstacle.0
                    && obstacle.0 >= target
                {
                    pos.0 = obstacle.0 + 1;
                } else {
                    pos.0 = target;
                }
            }
            Direction::Down => {
                let target = skipper.down[pos.0 as usize][pos.1 as usize];

                if pos.1 == obstacle.1
                    && pos.0 < obstacle.0
                    && obstacle.0 <= target
                {
                    pos.0 = obstacle.0 - 1;
                } else {
                    pos.0 = target;
                }
            }
            Direction::Left => {
                let target = skipper.left[pos.0 as usize][pos.1 as usize];

                if pos.0 == obstacle.0
                    && pos.1 > obstacle.1
                    && obstacle.1 >= target
                {
                    pos.1 = obstacle.1 + 1;
                } else {
                    pos.1 = target;
                }
            }
            Direction::Right => {
                let target = skipper.right[pos.0 as usize][pos.1 as usize];

                if pos.0 == obstacle.0
                    && pos.1 < obstacle.1
                    && obstacle.1 <= target
                {
                    pos.1 = obstacle.1 - 1;
                } else {
                    pos.1 = target;
                }
            }
        }

        dir = dir.rotate();
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
