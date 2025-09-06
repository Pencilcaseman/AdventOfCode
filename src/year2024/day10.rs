#![warn(clippy::pedantic, clippy::nursery)]

#[derive(Clone)]
pub struct Grid<T> {
    rows: usize,
    cols: usize,
    data: Vec<T>,
}

impl<T> Grid<T> {
    fn new(rows: usize, cols: usize) -> Self
    where
        T: Default + Copy,
    {
        Self { rows, cols, data: vec![T::default(); rows * cols] }
    }

    fn new_with(rows: usize, cols: usize, val: T) -> Self
    where
        T: Default + Copy,
    {
        Self { rows, cols, data: vec![val; rows * cols] }
    }

    fn row(&self, index: usize) -> &[T] {
        debug_assert!(
            index < self.rows,
            "Index {index} out of range for Grid with {} rows",
            self.rows
        );

        &self.data[(index * self.cols)..((index + 1) * self.cols)]
    }
}

impl<T> std::ops::Index<usize> for Grid<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        debug_assert!(
            index < self.rows * self.cols,
            "Index {} out of range for Grid with {} elements",
            index,
            self.rows * self.cols
        );
        &self.data[index]
    }
}

impl<T> std::ops::IndexMut<usize> for Grid<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        debug_assert!(
            index < self.rows * self.cols,
            "Index {} out of range for Grid with {} elements",
            index,
            self.rows * self.cols
        );
        &mut self.data[index]
    }
}

impl<T> std::ops::Index<(usize, usize)> for Grid<T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        debug_assert!(
            index.0 < self.rows,
            "Index {} out of range for Grid with {} rows",
            index.0,
            self.rows
        );
        debug_assert!(
            index.1 < self.cols,
            "Index {} out of range for Grid with {} cols",
            index.1,
            self.cols
        );
        &self.data[index.0 * self.cols + index.1]
    }
}

impl<T> std::ops::IndexMut<(usize, usize)> for Grid<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        debug_assert!(
            index.0 < self.rows,
            "Index {} out of range for Grid with {} rows",
            index.0,
            self.rows
        );
        debug_assert!(
            index.1 < self.cols,
            "Index {} out of range for Grid with {} cols",
            index.1,
            self.cols
        );
        &mut self.data[index.0 * self.cols + index.1]
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for Grid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for r in 0..self.rows {
            writeln!(f, "{:?}", self.row(r))?;
        }

        Ok(())
    }
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl std::ops::Add<Direction> for (usize, usize) {
    type Output = (usize, usize);

    fn add(self, rhs: Direction) -> Self::Output {
        match rhs {
            Direction::Up => (self.0.wrapping_sub(1), self.1),
            Direction::Down => (self.0.wrapping_add(1), self.1),
            Direction::Left => (self.0, self.1.wrapping_sub(1)),
            Direction::Right => (self.0, self.1.wrapping_add(1)),
        }
    }
}

type Input = Grid<u8>;

#[must_use]
pub fn parse(input: &str) -> Input {
    let cols = input.bytes().position(|b| b == b'\n').unwrap_or(1);
    let rows = input.len() / cols;

    let mut grid = Grid::new(rows, cols);

    let mut idx = 0;
    for byte in input.bytes() {
        if byte == b'\n' {
            continue;
        }

        grid[idx] = byte;
        idx += 1;
    }

    grid
}

pub fn dfs(
    input: &Input,
    seen: &mut Grid<u32>,
    id: u32,
    pos: (usize, usize),
    distinct: bool,
) -> u32 {
    let mut res = 0;

    for new in
        [Direction::Up, Direction::Down, Direction::Left, Direction::Right]
            .into_iter()
            .map(|d| pos + d)
    {
        if new.0 < input.rows
            && new.1 < input.cols
            && input[pos].wrapping_sub(input[new]) == 1
            && (!distinct || seen[new] != id)
        {
            seen[new] = id;

            if input[new] == b'0' {
                res += 1;
            } else {
                res += dfs(input, seen, id, new, distinct);
            }
        }
    }

    res
}

#[must_use]
pub fn solve(input: &Input, distinct: bool) -> u32 {
    let mut seen = Grid::<u32>::new_with(input.rows, input.cols, u32::MAX);
    let mut res = 0;

    for row in 0..input.rows {
        for col in 0..input.cols {
            if input[(row, col)] == b'9' {
                res += dfs(
                    input,
                    &mut seen,
                    u32::try_from(row * input.cols + col).unwrap_or(0),
                    (row, col),
                    distinct,
                );
            }
        }
    }

    res
}

#[must_use]
pub fn part1(input: &Input) -> u32 {
    solve(input, true)
}

#[must_use]
pub fn part2(input: &Input) -> u32 {
    solve(input, false)
}

// For my input, the correct answer is:
// Part 1: 816
// Part 2: 1960
