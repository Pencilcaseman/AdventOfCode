#![warn(clippy::pedantic, clippy::nursery)]

use std::collections::HashMap;

use rustc_hash::FxBuildHasher;

use crate::util::integer::{num_length, pow10};

type FastHashMap<K, V> = HashMap<K, V, FxBuildHasher>;

type Input = Vec<usize>;

fn split(num: usize) -> (usize, usize) {
    let len = num_length(num as u64);

    debug_assert!(len.is_multiple_of(2));

    let half = usize::try_from(pow10(len / 2)).unwrap();
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
    const PREALLOC: usize = 4096;

    // Maps from stone ID to table index
    let mut mapping = FastHashMap::<usize, usize>::default();

    // Stores the number of stones with a particular ID
    let mut count = Vec::with_capacity(PREALLOC);
    let mut new_count = Vec::with_capacity(PREALLOC);

    // Buffer
    let mut stones = Vec::with_capacity(PREALLOC);

    // New stones to process (double-buffered)
    let mut todo_front = Vec::with_capacity(PREALLOC);
    let mut todo_back = Vec::with_capacity(PREALLOC);

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
            } else if num_length(stone as u64).is_multiple_of(2) {
                let (first, second) = split(stone);
                (map_index(first), map_index(second))
            } else {
                (map_index(stone * 2024), usize::MAX)
            };

            stones.push(new_indices);
        }

        new_count.reserve_exact(mapping.len());
        unsafe {
            new_count.set_len(mapping.len());
        }
        new_count.fill(0);

        for (&(idx_1, idx_2), num) in stones.iter().zip(&count) {
            new_count[idx_1] += num;

            if idx_2 != usize::MAX {
                new_count[idx_2] += num;
            }
        }

        (count, new_count) = (new_count, count);
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
