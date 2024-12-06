#![warn(clippy::pedantic, clippy::nursery)]

use std::cmp::Ordering::*;

// Note that the numbers are all two digits long, so we can optimize the parsing
// quite a bit.

/// # Panics
///
/// Panics if the input is not valid
#[must_use]
pub fn parse(input: &str) -> (u32, u32) {
    // let mut pairs = Vec::with_capacity(1024);
    let bytes = input.as_bytes();
    let mut idx = 0;

    let mut map = [[Greater; 100]; 100];

    // Parse pairs
    loop {
        if bytes[idx] == b'\n' {
            idx += 1;
            break;
        }

        let n1 = (bytes[idx] - b'0') * 10 + (bytes[idx + 1] - b'0');
        let n2 = (bytes[idx + 3] - b'0') * 10 + (bytes[idx + 4] - b'0');

        idx += 6;

        // pairs.push((u32::from(n1), u32::from(n2)));
        map[n1 as usize][n2 as usize] = Less;
    }

    let mut part1 = 0;
    let mut part2 = 0;
    let mut section = Vec::new();

    while idx < bytes.len() {
        section.clear();

        loop {
            let n = (bytes[idx] - b'0') * 10 + (bytes[idx + 1] - b'0');
            idx += 2;

            section.push(u32::from(n));

            if idx >= bytes.len() || bytes[idx] == b'\n' {
                idx += 1;
                break;
            }
            idx += 1;
        }

        let mid = section.len() / 2;

        if section.is_sorted_by(|&a, &b| map[a as usize][b as usize] == Less) {
            part1 += section[mid];
        } else {
            section.select_nth_unstable_by(mid, |&a, &b| {
                map[a as usize][b as usize]
            });
            part2 += section[mid];
        }
    }

    (part1, part2)
}

// fn check_section(ordering: &[(u32, u32)], section: &[u32]) -> bool {
//     let mut map = [[std::cmp::Ordering::Greater; 100]; 100];
//
//     for (first, second) in ordering {
//         map[*first as usize][*second as usize] = std::cmp::Ordering::Less;
//     }
//
//     section.is_sorted_by(|a, b| {
//         map[*a as usize][*b as usize] == std::cmp::Ordering::Less
//     })
// }

// fn middle_sorted_solution(ordering: &[(u32, u32)], section: &[u32]) -> u32 {
//     let mut map = [[std::cmp::Ordering::Greater; 100]; 100];
//
//     for (first, second) in ordering {
//         map[*first as usize][*second as usize] = std::cmp::Ordering::Less;
//     }
//
//     section.select_nth_unstable_by(section.len() / 2, |a, b| {
//         map[*a as usize][*b as usize]
//     });
//
//     section[section.len() / 2]
// }

#[must_use]
pub fn part1(input: &(u32, u32)) -> u32 {
    // sections
    //     .iter()
    //     .filter(|section| check_section(pairs, section))
    //     .map(|valid| valid[valid.len() / 2])
    //     .sum()
    input.0
}

#[must_use]
pub fn part2(input: &(u32, u32)) -> u32 {
    input.1
}

// For my input, the correct answer is:
// Part 1: 5268
// Part 2:
