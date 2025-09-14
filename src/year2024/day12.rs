#![warn(clippy::pedantic, clippy::nursery)]

use num_traits::WrappingAdd;

use crate::util::{
    parse::grid_to_ndarray,
    point::{Direction, Point2D},
};

type Input = (u32, u32);

#[must_use]
pub fn parse(input: &str) -> Input {
    let grid = grid_to_ndarray(input);

    let rows = grid.dim().0;
    let cols = grid.dim().1;

    let mut open = Vec::with_capacity(1024);
    let mut edges = Vec::with_capacity(1024);
    let mut seen = ndarray::Array2::from_elem((rows, cols), false);

    let mut part1 = 0;
    let mut part2 = 0;

    for (plot, _) in grid.indexed_iter() {
        if seen[plot] {
            continue;
        }

        open.push(plot);

        let mut area = 0;
        let mut perimeter_1 = 0;
        let mut perimeter_2 = 0;

        let current = grid[plot];

        let check = |point: Point2D<usize>| {
            point.row < rows
                && point.col < cols
                && grid[point.tuple()] == current
        };

        while let Some(plot) = open.pop() {
            if seen[plot] {
                continue;
            }
            seen[plot] = true;

            area += 1;

            for dir in Direction::orthogonal() {
                let new_point = Point2D::<usize>::from(plot)
                    .wrapping_add(&Point2D::from(dir));

                if check(new_point) {
                    open.push(new_point.tuple());
                } else {
                    perimeter_1 += 1;
                    edges.push((Point2D::from(plot), dir));
                }
            }
        }

        for (plot, dir) in edges.drain(..) {
            let left = dir.counter_clockwise();
            let right = dir.clockwise();

            perimeter_2 += u32::from(
                !check(plot.wrapping_add_dir(&left))
                    || check(
                        plot.wrapping_add_dir(&dir).wrapping_add_dir(&left),
                    ),
            );
            perimeter_2 += u32::from(
                !check(plot.wrapping_add_dir(&right))
                    || check(
                        plot.wrapping_add_dir(&dir).wrapping_add_dir(&right),
                    ),
            );
        }

        part1 += area * perimeter_1;
        part2 += area * (perimeter_2 / 2);
    }

    (part1, part2)
}

#[must_use]
pub const fn part1(input: &Input) -> u32 {
    input.0
}

#[must_use]
pub const fn part2(input: &Input) -> u32 {
    input.1
}

// For my input, the correct answer is:
// Part 1: 1371306
// Part 2: 805880
