#![warn(clippy::pedantic, clippy::nursery)]

use std::collections::HashSet;

use fxhash::FxBuildHasher;

type FastHashSet<T> = HashSet<T, FxBuildHasher>;

type Input = (Vec<Vec<u8>>, (usize, usize), (usize, usize));

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

impl std::ops::Index<Direction> for [Vec<Vec<isize>>; 4] {
    type Output = Vec<Vec<isize>>;

    fn index(&self, dir: Direction) -> &Self::Output {
        match dir {
            Direction::Up => &self[0],
            Direction::Down => &self[1],
            Direction::Left => &self[2],
            Direction::Right => &self[3],
        }
    }
}

impl std::ops::IndexMut<Direction> for [Vec<Vec<isize>>; 4] {
    fn index_mut(&mut self, dir: Direction) -> &mut Self::Output {
        match dir {
            Direction::Up => &mut self[0],
            Direction::Down => &mut self[1],
            Direction::Left => &mut self[2],
            Direction::Right => &mut self[3],
        }
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

    (grid, pos, (rows, cols))
}

#[allow(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_possible_wrap,
    clippy::match_on_vec_items
)]
#[must_use]
pub fn part1((grid, pos, (rows, cols)): &Input) -> usize {
    let mut grid = grid.clone();
    let mut pos = (pos.0 as isize, pos.1 as isize);
    let mut dir = Direction::Up;

    let mut count = 1; // Include the start position

    loop {
        let next = pos + dir;

        if next.0 < 0
            || next.0 >= *rows as isize
            || next.1 < 0
            || next.1 >= *cols as isize
        {
            break;
        }

        match grid[next.0 as usize][next.1 as usize] {
            b'#' => {
                dir = dir.rotate();
                continue;
            }
            b'.' => {
                grid[next.0 as usize][next.1 as usize] = b'^';
                count += 1;
            }
            _ => {}
        }

        pos = next;
    }

    count
}

#[allow(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_possible_wrap,
    clippy::match_on_vec_items
)]
#[must_use]
pub fn part2((grid, pos, (rows, cols)): &Input) -> usize {
    let mut grid = grid.clone();
    let mut pos = (pos.0 as isize, pos.1 as isize);
    let mut dir = Direction::Up;
    let mut path = Vec::with_capacity(4096);

    while (pos + dir).0 >= 0
        && (pos + dir).0 < *rows as isize
        && (pos + dir).1 >= 0
        && (pos + dir).1 < *cols as isize
    {
        let tmp = pos + dir;
        if grid[tmp.0 as usize][tmp.1 as usize] == b'#' {
            dir = dir.rotate();
        }

        let next = pos + dir;

        if grid[next.0 as usize][next.1 as usize] == b'.' {
            path.push((pos, dir));
            grid[next.0 as usize][next.1 as usize] = b'^';
        }

        pos = next;
    }

    let mut count = 0;

    for (pos, dir) in &path {
        if is_loop(&mut grid, *pos, (*rows, *cols), *dir) {
            count += 1;
        }
    }

    count
}

#[allow(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_possible_wrap,
    clippy::match_on_vec_items
)]
#[must_use]
pub fn is_loop(
    grid: &mut [Vec<u8>],
    mut pos: (isize, isize),
    (rows, cols): (usize, usize),
    mut dir: Direction,
) -> bool {
    let mut seen =
        FastHashSet::with_capacity_and_hasher(4096, FxBuildHasher::default());

    let obstacle = pos + dir;
    let prev = grid[obstacle.0 as usize][obstacle.1 as usize];
    grid[obstacle.0 as usize][obstacle.1 as usize] = b'#';

    while (pos + dir).0 >= 0
        && (pos + dir).0 < rows as isize
        && (pos + dir).1 >= 0
        && (pos + dir).1 < cols as isize
    {
        if !seen.insert((pos, dir)) {
            grid[obstacle.0 as usize][obstacle.1 as usize] = prev;
            return true;
        }

        let tmp = pos + dir;
        if grid[tmp.0 as usize][tmp.1 as usize] == b'#' {
            dir = dir.rotate();
            continue;
        }

        let next = pos + dir;
        pos = next;
    }

    grid[obstacle.0 as usize][obstacle.1 as usize] = prev;
    false
}

// For my input, the correct answer is:
// Part 1: 5199
// Part 2: 1915
