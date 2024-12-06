#![warn(clippy::pedantic, clippy::nursery)]

use std::collections::HashMap;

use fxhash::FxBuildHasher;

use crate::util::parse;

type FastHashMap<K, V> = HashMap<K, V, FxBuildHasher>;

/// # Panics
///
/// Panics if the input is not valid
#[must_use]
pub fn parse(input: &str) -> (Vec<(u32, u32)>, Vec<Vec<u32>>) {
    let mut pairs = Vec::with_capacity(1024);
    let bytes = input.as_bytes();
    let mut idx = 0;

    // Parse pairs
    loop {
        let mut n1: u32 = 0;
        let mut n2: u32 = 0;

        if bytes[idx] == b'\n' {
            // Skip empty line
            idx += 1;
            break;
        }

        while bytes[idx].is_ascii_digit() {
            n1 = n1 * 10 + u32::from(bytes[idx] - b'0');
            idx += 1;
        }

        idx += 1; // Skip '|'

        while bytes[idx].is_ascii_digit() {
            n2 = n2 * 10 + u32::from(bytes[idx] - b'0');
            idx += 1;
        }
        idx += 1; // Skip newline

        pairs.push((n1, n2));
    }

    let mut sections: Vec<Vec<u32>> = Vec::with_capacity(1024);

    while idx < bytes.len() {
        let mut tmp = Vec::with_capacity(32);

        loop {
            let mut n = 0;

            while idx < bytes.len() && bytes[idx].is_ascii_digit() {
                n = n * 10 + u32::from(bytes[idx] - b'0');
                idx += 1;
            }

            tmp.push(n);

            if idx >= bytes.len() || bytes[idx] == b'\n' {
                idx += 1;
                break;
            }
            idx += 1;
        }

        sections.push(tmp);
    }

    (pairs, sections)
}

fn check_section(ordering: &[(u32, u32)], section: &[u32]) -> bool {
    let map: FastHashMap<u32, usize> =
        section.iter().enumerate().map(|(i, n)| (*n, i)).collect();

    ordering.iter().all(|(left, right)| {
        !(map.contains_key(left) && map.contains_key(right))
            || map[left] < map[right]
    })
}

#[must_use]
pub fn part1((pairs, sections): &(Vec<(u32, u32)>, Vec<Vec<u32>>)) -> u32 {
    sections
        .iter()
        .filter(|section| check_section(pairs, section))
        .map(|valid| valid[valid.len() / 2])
        .sum()
}

#[must_use]
pub fn part2(input: &(Vec<(u32, u32)>, Vec<Vec<u32>>)) -> u32 {
    0
}

// For my input, the correct answer is:
// Part 1: 5268
// Part 2:
