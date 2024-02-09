#![warn(clippy::pedantic, clippy::nursery)]

use atoi::FromRadix10;

const AROUND: [(i32, i32); 8] =
    [(-1, -1), (-1, 0), (-1, 1), (0, 1), (1, 1), (1, 0), (1, -1), (0, -1)];

#[derive(Debug, Copy, Clone)]
enum Item {
    Number(usize),
    Gear,
}

pub fn parse(input: &str) -> Vec<(usize, usize)> {
    let mut grid: Vec<Option<Item>> = vec![None; input.len()];

    let input = input.as_bytes();
    let mut index = 0;

    let line_length = input
        .iter()
        .enumerate()
        .find_map(|(index, &b)| if b == b'\n' { Some(index) } else { None })
        .unwrap()
        + 1;

    while index < input.len() {
        match input[index..][0] {
            b'0'..=b'9' => {
                let (num, used) = usize::from_radix_10(&input[index..]);
                for _ in 0..used {
                    grid[index] = Some(Item::Number(num));
                    index += 1;
                }
            }
            b'\n' | b'.' => index += 1,
            _ => {
                grid[index] = Some(Item::Gear);
                index += 1;
            }
        }
    }

    let mut result = Vec::new();

    for index in 0..grid.len() {
        if matches!(grid[index], Some(Item::Gear)) {
            let mut res = (0, 0);

            for (r, c) in &AROUND {
                let shift = r * line_length as i32 + c;
                let new = (index as i32).wrapping_add(shift) as usize;

                if new < grid.len() {
                    if let Some(Item::Number(n)) = grid[new] {
                        if res.0 == 0 {
                            res.0 = n;
                        } else if n != res.0 {
                            res.1 = n;
                            break;
                        }
                    }
                }
            }

            if res.0 != 0 {
                result.push(res);
            }
        }
    }

    result
}

pub fn part1(input: &[(usize, usize)]) -> usize {
    input.iter().map(|(a, b)| a + b).sum()
}

pub fn part2(input: &[(usize, usize)]) -> usize {
    input.iter().map(|(a, b)| a * b).sum()
}

// For my input, the correct answer is:
// Part 1: 527364
// Part 2: 79026871
