#![warn(clippy::pedantic, clippy::nursery)]

use num_traits::WrappingAdd;

use crate::util::{
    parse::grid_to_ndarray,
    point::{Direction, Point2D},
};

type Input = (usize, usize);

#[must_use]
pub fn parse(input: &str) -> Input {
    let grid = grid_to_ndarray(input.bytes());

    let rows = grid.dim().0;
    let cols = grid.dim().1;

    let mut open = Vec::with_capacity(1024);
    let mut seen = ndarray::Array2::from_elem((rows, cols), false);

    let mut part1 = 0;
    let mut part2 = 0;

    for (plot, _) in grid.indexed_iter() {
        unsafe {
            if *seen.uget(plot) {
                continue;
            }
        }

        let mut area = 0;
        let mut perimeter_1 = 0;
        let mut perimeter_2 = 0;

        let current = unsafe { grid.uget(plot) };

        let check = |point: Point2D<usize>| unsafe {
            point.row < rows
                && point.col < cols
                && grid.uget(point.tuple()) == current
        };

        open.push(Point2D::from(plot));
        unsafe {
            *seen.uget_mut(plot) = true;
        }

        while let Some(plot) = open.pop() {
            area += 1;

            for dir in Direction::orthogonal() {
                let new_plot = plot.wrapping_add(&Point2D::from(dir));

                unsafe {
                    if check(new_plot) {
                        if !seen.uget(new_plot.tuple()) {
                            *seen.uget_mut(new_plot.tuple()) = true;
                            open.push(new_plot);
                        }
                    } else {
                        perimeter_1 += 1;

                        let left = dir.counter_clockwise();
                        let right = dir.clockwise();

                        perimeter_2 += usize::from(
                            !check(plot.wrapping_add_dir(&left))
                                || check(
                                    plot.wrapping_add_dir(&dir)
                                        .wrapping_add_dir(&left),
                                ),
                        );
                        perimeter_2 += usize::from(
                            !check(plot.wrapping_add_dir(&right))
                                || check(
                                    plot.wrapping_add_dir(&dir)
                                        .wrapping_add_dir(&right),
                                ),
                        );
                    }
                }
            }
        }

        part1 += area * perimeter_1;
        part2 += area * (perimeter_2 / 2);
    }

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

// For my input, the correct answer is:
// Part 1: 1371306
// Part 2: 805880
