#![warn(clippy::pedantic, clippy::nursery)]

use std::collections::HashMap;

use fxhash::FxBuildHasher;

type FastHashMap<K, V> = HashMap<K, V, FxBuildHasher>;

// Note that the numbers are all two digits long, so we can optimize the parsing
// quite a bit.

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
        if bytes[idx] == b'\n' {
            idx += 1;
            break;
        }

        let n1 = (bytes[idx] - b'0') * 10 + (bytes[idx + 1] - b'0');
        let n2 = (bytes[idx + 3] - b'0') * 10 + (bytes[idx + 4] - b'0');

        idx += 6;

        pairs.push((u32::from(n1), u32::from(n2)));
    }

    let mut sections: Vec<Vec<u32>> = Vec::with_capacity(1024);

    while idx < bytes.len() {
        let mut tmp = Vec::with_capacity(32);

        loop {
            let n = (bytes[idx] - b'0') * 10 + (bytes[idx + 1] - b'0');
            idx += 2;

            tmp.push(u32::from(n));

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
    let mut map = [[std::cmp::Ordering::Greater; 100]; 100];

    for (first, second) in ordering {
        map[*first as usize][*second as usize] = std::cmp::Ordering::Less;
    }

    section.is_sorted_by(|a, b| {
        map[*a as usize][*b as usize] == std::cmp::Ordering::Less
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
