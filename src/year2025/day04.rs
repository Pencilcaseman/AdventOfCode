use ndarray::Array2;
use num_traits::WrappingAdd;

use crate::util::{parse::grid_to_ndarray, point::Point2D};

type Input = Array2<u8>;

pub fn parse(input: &str) -> Input {
    grid_to_ndarray(input.bytes())
}

pub fn part1(input: &Input) -> usize {
    let offsets = [
        Point2D::new(-1, -1), // up left
        Point2D::new(-1, 0),  // up
        Point2D::new(-1, 1),  // up right
        //
        Point2D::new(0, -1), // left
        Point2D::new(0, 1),  // right
        //
        Point2D::new(1, -1), // down left
        Point2D::new(1, 0),  // down
        Point2D::new(1, 1),  // down right
    ];

    let mut grid = Array2::<u8>::zeros(input.dim());

    let f = |(row, col): (i64, i64)| {
        offsets
            .iter()
            .map(|offset| {
                let pos = Point2D::new(row, col).wrapping_add(offset);

                if pos.row < 0
                    || pos.col < 0
                    || pos.row >= input.dim().0 as i64
                    || pos.col >= input.dim().1 as i64
                {
                    0
                } else {
                    match input[(pos.row as usize, pos.col as usize)] {
                        b'@' => 1,
                        _ => 0,
                    }
                }
            })
            .sum::<u8>()
    };

    grid.indexed_iter_mut().for_each(|((row, col), val)| {
        if input[(row, col)] == b'@' {
            *val = f((row as i64, col as i64))
        } else {
            *val = u8::MAX;
        }
    });

    grid.iter().filter(|&&v| v < 4).count()
}

pub fn part2(input: &Input) -> u64 {
    let offsets = [
        Point2D::new(-1, -1), // up left
        Point2D::new(-1, 0),  // up
        Point2D::new(-1, 1),  // up right
        //
        Point2D::new(0, -1), // left
        Point2D::new(0, 1),  // right
        //
        Point2D::new(1, -1), // down left
        Point2D::new(1, 0),  // down
        Point2D::new(1, 1),  // down right
    ];

    let mut grid = Array2::<u8>::zeros(input.dim());

    let f = |(row, col): (i64, i64)| {
        offsets
            .iter()
            .map(|offset| {
                let pos = Point2D::new(row, col).wrapping_add(offset);

                if pos.row < 0
                    || pos.col < 0
                    || pos.row >= input.dim().0 as i64
                    || pos.col >= input.dim().1 as i64
                {
                    0
                } else {
                    match input[(pos.row as usize, pos.col as usize)] {
                        b'@' => 1,
                        _ => 0,
                    }
                }
            })
            .sum::<u8>()
    };

    grid.indexed_iter_mut().for_each(|((row, col), val)| {
        if input[(row, col)] == b'@' {
            *val = f((row as i64, col as i64))
        } else {
            *val = u8::MAX;
        }
    });

    // Grid now contains the initial counts

    let mut total_removed = 0;
    loop {
        let mut step_removed = 0;

        let rows = grid.dim().0;
        let cols = grid.dim().1;

        for pos in ndarray::indices(grid.dim()) {
            if grid[pos] < 4 {
                // Paper can be removed, so decrement surrounding paper
                grid[pos] = u8::MAX;
                step_removed += 1;

                offsets
                    .iter()
                    .filter_map(|o| {
                        let new = o.wrapping_add(&Point2D::new(
                            pos.0 as i64,
                            pos.1 as i64,
                        ));

                        if new.row < 0
                            || new.col < 0
                            || new.row >= rows as i64
                            || new.col >= cols as i64
                        {
                            None
                        } else {
                            Some(new)
                        }
                    })
                    .for_each(|p| {
                        grid[(p.row as usize, p.col as usize)] -= 1;
                    })
            }
        }

        total_removed += step_removed;

        if step_removed == 0 {
            break;
        }
    }

    total_removed
}

// Answers for my input:
// Part 1: 1553
// Part 2: 8442
