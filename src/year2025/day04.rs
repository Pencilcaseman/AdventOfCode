use ndarray::Array2;

use crate::util::parse::grid_to_ndarray;

type Input = (usize, usize);

pub fn parse(input: &str) -> Input {
    let input = grid_to_ndarray(input.bytes());

    let dim = input.dim();
    let mut count_grid = Array2::<u8>::zeros(dim);

    let offsets = [
        (usize::MAX, usize::MAX), // -1 -1
        (usize::MAX, 0),          // -1 0
        (usize::MAX, 1),          // -1 1
        (0, usize::MAX),          // 0 -1
        (0, 1),                   // 0 1
        (1, usize::MAX),          // 1 -1
        (1, 0),                   // 1 0
        (1, 1),                   // 1 1
    ];

    let f = |(row, col): (usize, usize)| {
        offsets
            .into_iter()
            .map(|offset| {
                let pos =
                    (row.wrapping_add(offset.0), col.wrapping_add(offset.1));

                if pos.0 >= input.dim().0 || pos.1 >= input.dim().1 {
                    0
                } else {
                    match input[pos] {
                        b'@' => 1,
                        _ => 0,
                    }
                }
            })
            .sum::<u8>()
    };

    count_grid.indexed_iter_mut().for_each(|(pos, val)| {
        if input[pos] == b'@' {
            *val = f(pos)
        } else {
            *val = u8::MAX;
        }
    });

    let part1 = input
        .iter()
        .zip(&count_grid)
        .filter(|(b, c)| **b == b'@' && **c < 4)
        .count();

    let mut total_removed = 0;

    loop {
        let mut step_removed = 0;

        for pos in ndarray::indices(dim) {
            if count_grid[pos] < 4 {
                // Paper can be removed, so decrement surrounding paper
                count_grid[pos] = u8::MAX;
                step_removed += 1;

                offsets
                    .iter()
                    .filter_map(|o| {
                        let new =
                            (pos.0.wrapping_add(o.0), pos.1.wrapping_add(o.1));

                        if new.0 >= dim.0 || new.1 >= dim.1 {
                            None
                        } else {
                            Some(new)
                        }
                    })
                    .for_each(|p| {
                        count_grid[p] -= 1;
                    })
            }
        }

        total_removed += step_removed;

        if step_removed == 0 {
            break;
        }
    }

    (part1, total_removed)
}

pub fn part1(input: &Input) -> usize {
    input.0
}

pub fn part2(input: &Input) -> usize {
    input.1
}

// Answers for my input:
// Part 1: 1553
// Part 2: 8442
