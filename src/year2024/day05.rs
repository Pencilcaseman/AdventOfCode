#![warn(clippy::pedantic, clippy::nursery)]

use std::cmp::Ordering::{Greater, Less};

// Note that the numbers are all two digits long, so we can optimize the parsing
// quite a bit.

#[must_use]
pub fn parse(input: &str) -> (u32, u32) {
    let bytes = input.as_bytes();
    let mut idx = 0;

    let mut map = [[Greater; 100]; 100];

    loop {
        if bytes[idx] == b'\n' {
            idx += 1;
            break;
        }

        let n1 = (bytes[idx] - b'0') * 10 + (bytes[idx + 1] - b'0');
        let n2 = (bytes[idx + 3] - b'0') * 10 + (bytes[idx + 4] - b'0');
        idx += 6;

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

#[must_use]
pub const fn part1(input: &(u32, u32)) -> u32 {
    input.0
}

#[must_use]
pub const fn part2(input: &(u32, u32)) -> u32 {
    input.1
}

// For my input, the correct answer is:
// Part 1: 5268
// Part 2: 5799
