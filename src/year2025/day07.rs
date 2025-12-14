use ndarray::Array2;

use crate::util::parse::grid_to_ndarray;

type Input = Array2<u8>;

pub fn parse(input: &str) -> Input {
    grid_to_ndarray(input.bytes())
}

pub fn part1(input: &Input) -> i32 {
    let mut current = vec![0; input.dim().1];
    let mut total = 0;

    for row in input.rows() {
        for i in 0..input.dim().1 {
            match row[i] {
                b'.' => (),
                b'S' => {
                    current[i] = 1;
                }
                b'^' => {
                    if current[i] == 1 {
                        current[i] = 0;
                        current[i - 1] = 1;
                        current[i + 1] = 1;
                        total += 1;
                    }
                }
                _ => unreachable!(),
            }
        }
    }

    total
}

pub fn part2(input: &Input) -> u64 {
    let mut current = vec![0u64; input.dim().1];

    for row in input.rows() {
        for i in 0..input.dim().1 {
            match row[i] {
                b'.' => (),
                b'S' => {
                    current[i] = 1;
                }
                b'^' => {
                    if current[i] > 0 {
                        current[i - 1] += current[i];
                        current[i + 1] += current[i];
                        current[i] = 0;
                    }
                }
                _ => unreachable!(),
            }
        }
    }

    current.into_iter().sum()
}

// Answers for my input:
// Part 1: 1662
// Part 2: 40941112789504
