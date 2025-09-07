#![warn(clippy::pedantic, clippy::nursery)]

use std::collections::HashMap;

use rustc_hash::FxBuildHasher;

type FastHashMap<K, V> = HashMap<K, V, FxBuildHasher>;

type Input = Vec<usize>;

/// # Panics
#[must_use]
pub fn parse(input: &str) -> Input {
    input.split_whitespace().map(|s| s.parse().unwrap()).collect()
}

fn split(num: usize) -> (usize, usize) {
    let len = num.ilog10() + 1;

    debug_assert!(len.is_multiple_of(2));

    let half = 10usize.pow(len / 2);
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
        } else if !stone.ilog10().is_multiple_of(2) {
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

fn solve(input: &Input, times: usize) -> usize {
    let mut lookup = FastHashMap::default();
    input.iter().map(|stone| blink_stone(&mut lookup, *stone, times)).sum()
}

#[must_use]
pub fn part1(input: &Input) -> usize {
    solve(input, 25)
}

#[must_use]
pub fn part2(input: &Input) -> usize {
    solve(input, 75)
}

// For my input, the correct answer is:
// Part 1: 200446
// Part 2: 238317474993392
