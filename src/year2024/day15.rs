#![warn(clippy::pedantic, clippy::nursery)]

use crate::util::{
    parse::grid_to_ndarray,
    point::{Direction, Point2D},
};

const EMPTY: u8 = b'.';
const BOX: u8 = b'O';
const WALL: u8 = b'#';
const ROBOT: u8 = b'@';

type Grid = ndarray::Array2<u8>;
type Input<'a> = (Grid, &'a [u8], Point2D<usize>);

fn print_grid(grid: &Grid, robot: (usize, usize)) {
    for i in 0..grid.dim().0 {
        for j in 0..grid.dim().1 {
            if (i, j) == robot {
                print!("@");
            } else {
                print!("{}", grid[(i, j)] as char);
            }
        }
        println!();
    }
}

/// # Panics
#[must_use]
pub fn parse(input: &str) -> Input<'_> {
    let bytes = input.as_bytes();

    let mut grid = grid_to_ndarray(bytes.iter());
    let dirs = &bytes[grid.dim().0 * (grid.dim().1 + 1)..];

    let origin = grid
        .indexed_iter()
        .find_map(|(idx, v)| if *v == ROBOT { Some(idx) } else { None })
        .unwrap();

    grid[origin] = EMPTY;

    (grid, dirs, Point2D::new(origin.0, origin.1))
}

#[must_use]
pub fn part1((grid, dirs, origin): &Input) -> usize {
    let mut grid = grid.clone();
    let mut pos = *origin;

    let mut push = |pos: Point2D<usize>, dir: Direction| {
        let mut tmp_pos = pos + dir;

        let mut boxes = 0;

        while grid[tmp_pos.tuple()] != EMPTY && grid[tmp_pos.tuple()] != WALL {
            tmp_pos += dir;
            boxes += 1;
        }

        if grid[tmp_pos.tuple()] == EMPTY {
            let mut prev = EMPTY;
            tmp_pos = pos + dir;

            for _ in 0..=boxes {
                std::mem::swap(&mut prev, &mut grid[tmp_pos.tuple()]);
                tmp_pos += dir;
            }

            pos + dir
        } else {
            pos
        }
    };

    for &byte in *dirs {
        pos = match byte {
            b'^' => push(pos, Direction::Up),
            b'v' => push(pos, Direction::Down),
            b'<' => push(pos, Direction::Left),
            b'>' => push(pos, Direction::Right),
            _ => pos,
        }
    }

    grid.indexed_iter()
        .map(|(idx, v)| if *v == BOX { idx.0 * 100 + idx.1 } else { 0 })
        .sum()
}

#[must_use]
pub fn part2(input: &Input) -> u32 {
    0
}

// For my input, the correct answer is:
// Part 1: 1318523
// Part 2:
