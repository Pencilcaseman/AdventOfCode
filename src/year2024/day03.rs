#![warn(clippy::pedantic, clippy::nursery)]

#[must_use]
pub fn parse(input: &str) -> (u32, u32) {
    let mut part1 = 0;
    let mut part2 = 0;
    let mut enabled = true;

    let bytes = input.as_bytes();
    let mut idx = 0;

    while idx < bytes.len() {
        let byte = bytes[idx];

        // Skip invalid commands
        if byte != b'm' && byte != b'd' {
            idx += 1;
            continue;
        }

        let tmp = &bytes[idx..];
        if tmp.starts_with(b"mul(") {
            idx += 4;
        } else if tmp.starts_with(b"do()") {
            idx += 4;
            enabled = true;
            continue;
        } else if tmp.starts_with(b"don't()") {
            idx += 7;
            enabled = false;
            continue;
        } else {
            idx += 1;
            continue;
        }

        // Here we do not need to check if the digits exist, because the
        // multiplication will give zero if they don't. When summed, these will
        // not affect the output.

        let mut n1: u32 = 0;
        while bytes[idx].is_ascii_digit() {
            n1 = n1 * 10 + u32::from(bytes[idx].wrapping_sub(b'0'));
            idx += 1;
        }

        if bytes[idx] != b',' {
            continue;
        }
        idx += 1;

        let mut n2: u32 = 0;
        while bytes[idx].is_ascii_digit() {
            n2 = n2 * 10 + u32::from(bytes[idx].wrapping_sub(b'0'));
            idx += 1;
        }

        if bytes[idx] != b')' {
            continue;
        }

        let prod = n1 * n2;
        part1 += prod;
        if enabled {
            part2 += prod;
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
// Part 1: 164730528
// Part 2: 70478672
