#![warn(clippy::pedantic, clippy::nursery)]

use std::collections::HashMap;

use fxhash::FxBuildHasher;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MapItem {
    Empty,
    Wall,
    Start,
    Seen,
}

type Input = (Vec<Vec<MapItem>>, (isize, isize));

#[allow(
    clippy::cast_sign_loss,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap
)]
#[must_use]
pub fn parse(input: &str) -> Input {
    let mut start = (0, 0);

    let map = input
        .as_bytes()
        .split(|&b| b == b'\n')
        .enumerate()
        .map(|(row, line)| {
            line.iter()
                .enumerate()
                .map(|(col, c)| match c {
                    b'.' => MapItem::Empty,
                    b'#' => MapItem::Wall,
                    b'^' => {
                        start = (row as isize, col as isize);
                        MapItem::Start
                    }
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();

    (map, start)
}

// Rotates a direction 90 degrees clockwise
fn rotate(dir: (isize, isize)) -> (isize, isize) {
    (dir.1, -dir.0)
}

#[allow(
    clippy::cast_sign_loss,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap
)]
#[must_use]
pub fn part1(input: &Input) -> usize {
    let mut map: Vec<Vec<MapItem>> = input.0.clone();
    let mut pos = input.1;
    let mut dir = (-1, 0);
    let mut count = 0;

    loop {
        match map
            .get_mut(pos.0 as usize)
            .and_then(|row| row.get_mut(pos.1 as usize))
        {
            Some(MapItem::Wall) => {
                // Go backwards
                pos.0 -= dir.0;
                pos.1 -= dir.1;

                // Rotate
                dir = rotate(dir);

                // Go forwards
                pos.0 += dir.0;
                pos.1 += dir.1;
            }
            Some(item) => {
                // Empty, Start or Seen
                match item {
                    MapItem::Empty | MapItem::Start => {
                        pos.0 += dir.0;
                        pos.1 += dir.1;
                        count += 1;
                        *item = MapItem::Seen;
                    }
                    MapItem::Seen => {
                        pos.0 += dir.0;
                        pos.1 += dir.1;
                    }
                    MapItem::Wall => unreachable!(),
                }
            }
            None => break,
        }
    }

    count
}

#[must_use]
pub const fn part2(input: &Input) -> usize {
    0
    // todo!()
}

// For my input, the correct answer is:
// Part 1:
// Part 2:
