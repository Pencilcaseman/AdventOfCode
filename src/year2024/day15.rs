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
type Input = (Grid, Vec<(Direction, u8)>, Point2D<usize>);

/// # Panics
#[must_use]
pub fn parse(input: &str) -> Input {
    let mut bytes = input.bytes();

    let mut grid = grid_to_ndarray(&mut bytes);
    let (mut dirs, last) = bytes.fold(
        (Vec::with_capacity(1 << 16), (None, 0u8)),
        |(mut vec, prev), byte| {
            let dir = match byte {
                b'\n' => return (vec, prev),
                b'<' => Direction::Left,
                b'>' => Direction::Right,
                b'^' => Direction::Up,
                b'v' => Direction::Down,
                _ => unreachable!(),
            };

            if prev.0.is_none_or(|p| p == dir) {
                (vec, (Some(dir), prev.1 + 1))
            } else {
                vec.push((prev.0.unwrap(), prev.1));
                (vec, (Some(dir), 1))
            }
        },
    );

    dirs.push((last.0.unwrap(), last.1));

    let origin = grid
        .indexed_iter()
        .find_map(|(idx, v)| if *v == ROBOT { Some(idx) } else { None })
        .unwrap();

    // Keep track of the robot explicitly
    grid[origin] = EMPTY;

    (grid, dirs, Point2D::new(origin.0, origin.1))
}

#[must_use]
pub fn part1((grid, dirs, origin): &Input) -> usize {
    let mut grid = grid.clone();
    let mut pos = *origin;

    for &dir in dirs {
        let mut tmp_pos = pos;
        let mut empty = 0;
        let mut boxes = 0;

        loop {
            tmp_pos += dir.0;

            match grid[tmp_pos.tuple()] {
                WALL => {
                    break;
                }
                BOX => {
                    boxes += 1;
                }
                EMPTY => {
                    empty += 1;
                }
                _ => unreachable!(),
            }

            if empty == dir.1 {
                break;
            }
        }

        tmp_pos = pos;

        for _ in 0..usize::from(empty) {
            tmp_pos += dir.0;
            grid[tmp_pos.tuple()] = EMPTY;
        }

        pos = tmp_pos;

        for _ in 0..boxes {
            tmp_pos += dir.0;
            grid[tmp_pos.tuple()] = BOX;
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
