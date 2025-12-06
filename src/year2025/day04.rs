use ndarray::Array2;

use crate::util::parse::grid_to_ndarray;

type Input = (Vec<(usize, usize)>, Array2<u8>);

const OFFSETS: [(usize, usize); 8] = [
    (usize::MAX, usize::MAX), // -1 -1
    (usize::MAX, 0),          // -1 0
    (usize::MAX, 1),          // -1 1
    (0, usize::MAX),          // 0 -1
    (0, 1),                   // 0 1
    (1, usize::MAX),          // 1 -1
    (1, 0),                   // 1 0
    (1, 1),                   // 1 1
];

pub fn parse(input: &str) -> Input {
    let input = grid_to_ndarray(input.bytes());

    let dim = input.dim();
    let mut todo = Vec::new();
    let mut count_grid = Array2::<u8>::zeros(dim);

    let f = |(row, col): (usize, usize)| {
        OFFSETS
            .into_iter()
            .map(|offset| {
                let pos =
                    (row.wrapping_add(offset.0), col.wrapping_add(offset.1));

                if pos.0 >= dim.0 || pos.1 >= dim.1 {
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
            *val = f(pos);

            if *val < 4 {
                todo.push(pos);
            }
        } else {
            *val = u8::MAX;
        }
    });

    (todo, count_grid)
}

pub fn part1(input: &Input) -> usize {
    input.0.len()
}

pub fn part2(input: &Input) -> usize {
    let (mut todo, mut count_grid) = input.clone();
    let mut total_removed = 0;

    let dim = count_grid.dim();

    while let Some(pos) = todo.pop() {
        total_removed += 1;

        OFFSETS.into_iter().for_each(|offset| {
            let new =
                (pos.0.wrapping_add(offset.0), pos.1.wrapping_add(offset.1));

            if new.0 < dim.0 && new.1 < dim.1 {
                if count_grid[new] == 4 {
                    todo.push(new);
                }

                count_grid[new] -= 1;
            }
        });
    }

    total_removed
}

// Answers for my input:
// Part 1: 1553
// Part 2: 8442
