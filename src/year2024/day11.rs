#![warn(clippy::pedantic, clippy::nursery)]

use std::collections::HashMap;

use rustc_hash::FxBuildHasher;

type FastHashMap<K, V> = HashMap<K, V, FxBuildHasher>;

type Input = Vec<usize>;

// For our use case, it is faster to store a static array than to use ilog10
// and/or pow
const POW_10: [usize; 20] = [
    1,
    10,
    100,
    1_000,
    10_000,
    100_000,
    1_000_000,
    10_000_000,
    100_000_000,
    1_000_000_000,
    10_000_000_000,
    100_000_000_000,
    1_000_000_000_000,
    10_000_000_000_000,
    100_000_000_000_000,
    1_000_000_000_000_000,
    10_000_000_000_000_000,
    100_000_000_000_000_000,
    1_000_000_000_000_000_000,
    10_000_000_000_000_000_000,
];

fn num_length(num: usize) -> usize {
    for (exp, pow) in POW_10.iter().enumerate() {
        if num < *pow {
            return exp;
        }
    }

    0
}

const fn pow10(exp: usize) -> usize {
    POW_10[exp]
}

fn split(num: usize) -> (usize, usize) {
    let len = num_length(num);

    debug_assert!(len.is_multiple_of(2));

    let half = pow10(len / 2);
    let head = num / half;
    let tail = num - (head * half);

    (head, tail)
}

/// # Panics
#[must_use]
pub fn parse(input: &str) -> Input {
    input.split_whitespace().map(|s| s.parse().unwrap()).collect()
}

fn blink_n(input: &Input, times: usize) -> usize {
    // Maps from stone ID to table index
    let mut mapping = FastHashMap::<usize, usize>::default();

    // Stores the number of stones with a particular ID
    let mut count = Vec::with_capacity(1 << 12);

    // Buffer
    let mut stones = Vec::with_capacity(1 << 12);

    // New stones to process (double-buffered)
    let mut todo_front = Vec::with_capacity(1 << 12);
    let mut todo_back = Vec::with_capacity(1 << 12);

    for &stone in input {
        if let Some(&idx) = mapping.get(&stone) {
            count[idx] += 1;
        } else {
            count.push(1);
            todo_back.push(stone);
            mapping.insert(stone, mapping.len());
        }
    }

    for _ in 0..times {
        (todo_front, todo_back) = (todo_back, todo_front);

        let mut map_index = |stone| {
            let idx = mapping.len();
            *mapping.entry(stone).or_insert_with(|| {
                todo_back.push(stone);
                idx
            })
        };

        #[allow(clippy::iter_with_drain)]
        for stone in todo_front.drain(..) {
            let new_indices = if stone == 0 {
                (map_index(1), usize::MAX)
            } else if num_length(stone).is_multiple_of(2) {
                let (first, second) = split(stone);
                (map_index(first), map_index(second))
            } else {
                (map_index(stone * 2024), usize::MAX)
            };

            stones.push(new_indices);
        }

        let mut new_count = vec![0; mapping.len()];

        for (&(idx_1, idx_2), num) in stones.iter().zip(&count) {
            new_count[idx_1] += num;

            if idx_2 != usize::MAX {
                new_count[idx_2] += num;
            }
        }

        count = new_count;
    }

    count.into_iter().sum()
}

#[must_use]
pub fn part1(input: &Input) -> usize {
    blink_n(input, 25)
}

#[must_use]
pub fn part2(input: &Input) -> usize {
    blink_n(input, 75)
}

// For my input, the correct answer is:
// Part 1: 200446
// Part 2: 238317474993392
