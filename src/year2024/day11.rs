#![warn(clippy::pedantic, clippy::nursery)]

use std::collections::HashMap;

use rustc_hash::FxBuildHasher;

type FastHashMap<K, V> = HashMap<K, V, FxBuildHasher>;

type Input = Vec<usize>;

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

/// # Panics
#[must_use]
pub fn parse(input: &str) -> Input {
    input.split_whitespace().map(|s| s.parse().unwrap()).collect()
}

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

fn blink_stone(
    lookup: &mut FastHashMap<(usize, usize), usize>,
    stone: usize,
    times: usize,
) -> usize {
    if times == 0 {
        1
    } else if let Some(res) = lookup.get(&(stone, times)) {
        *res
    } else {
        let res = if stone == 0 {
            blink_stone(lookup, 1, times - 1)
        } else if num_length(stone).is_multiple_of(2) {
            let (head, tail) = split(stone);
            blink_stone(lookup, head, times - 1)
                + blink_stone(lookup, tail, times - 1)
        } else {
            blink_stone(lookup, stone * 2024, times - 1)
        };

        lookup.insert((stone, times), res);
        res
    }
}

fn blink_v2(input: &Input, times: usize) -> usize {
    // Maps from stone ID to table index
    let mut mapping = FastHashMap::<usize, usize>::default();

    // Stores the number of stones with a particular ID
    let mut count = Vec::with_capacity(1 << 12);

    // Stores the values of all the stones we have seen
    let mut numbers = Vec::with_capacity(1 << 12);

    // Stores some stuff
    let mut stones = Vec::with_capacity(1 << 12);

    // --------

    // New stones to process
    let mut todo_stones: Vec<usize> = Vec::with_capacity(1 << 12);

    let mut tmp = Vec::new();

    for &stone in input {
        if let Some(&idx) = mapping.get(&stone) {
            count[idx] += 1;
        } else {
            count.push(1);
            numbers.push(stone);
            tmp.push(stone);
            mapping.insert(stone, mapping.len());
        }
    }

    for _ in 0..times {
        (todo_stones, tmp) = (tmp, todo_stones);

        let mut stoner_fn = |stone| {
            let idx = mapping.len();
            *mapping.entry(stone).or_insert_with(|| {
                tmp.push(stone);
                idx
            })
        };

        for stone in todo_stones.drain(..) {
            let new_indices = if stone == 0 {
                (stoner_fn(1), usize::MAX)
            } else if num_length(stone).is_multiple_of(2) {
                let (first, second) = split(stone);
                (stoner_fn(first), stoner_fn(second))
            } else {
                (stoner_fn(stone * 2024), usize::MAX)
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

        // println!("Count: {count:?}");
        // println!("Stones: {numbers:?}");
        // println!("Mapping: {mapping:#?}");
        // println!("New Count: {new_count:?}");
        // println!("Todo Stones: {todo_stones:?}");

        count = new_count;
    }

    count.into_iter().sum()
}

fn solve(input: &Input, times: usize) -> usize {
    let mut lookup = FastHashMap::default();
    input.iter().map(|stone| blink_stone(&mut lookup, *stone, times)).sum()
}

#[must_use]
pub fn part1(input: &Input) -> usize {
    // solve(input, 25)
    blink_v2(input, 25)
}

#[must_use]
pub fn part2(input: &Input) -> usize {
    // solve(input, 75)
    blink_v2(input, 75)
}

// For my input, the correct answer is:
// Part 1: 200446
// Part 2: 238317474993392
