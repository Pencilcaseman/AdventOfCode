#![warn(clippy::pedantic, clippy::nursery)]

const NUMS: [&[u8]; 9] = [
    b"one", b"two", b"three", b"four", b"five", b"six", b"seven", b"eight",
    b"nine",
];

pub fn parse(input: &str) -> Vec<&str> {
    input.lines().collect()
}

/// Since all the values are 2-digit numbers, we just need to find the first and last digit,
/// multiply the first by 10 and add the second.
pub fn part1(input: &[&str]) -> u32 {
    input
        .iter()
        .map(|line| {
            let first = line
                .bytes()
                .find(u8::is_ascii_digit) // Find digits
                .map(|c| c - b'0') // Convert to a 'number'
                .unwrap();

            let second = line
                .bytes()
                .rfind(u8::is_ascii_digit)
                .map(|c| c - b'0')
                .unwrap();

            (first * 10 + second) as u32
        })
        .sum()
}

/// A helper function which returns the number (digit or string) at the start of a given string
#[inline(always)]
fn num(line: &[u8]) -> Option<u32> {
    if line[0].is_ascii_digit() {
        Some((line[0] - b'0') as u32)
    } else {
        NUMS.iter().enumerate().find_map(|(index, &n)| {
            if line.starts_with(n) {
                Some(index as u32 + 1)
            } else {
                None
            }
        })
    }
}

/// This is similar to part 1, except we have to search for digit strings as well. Iterating
/// backwards still works here because we either find a digit, the start of a digit string, or
/// nothing at all. In any case, the `num` function will work and the same logic as part 1 applies.
pub fn part2(input: &[&str]) -> u32 {
    input
        .iter()
        .map(|l| l.as_bytes())
        .map(|line| {
            let first = line
                .iter()
                .enumerate()
                .find_map(|(index, _)| num(&line[index..]))
                .unwrap();

            let second = line
                .iter()
                .enumerate()
                .rev()
                .find_map(|(index, _)| num(&line[index..]))
                .unwrap();

            (first * 10 + second) as u32
        })
        .sum()
}

// For my input, the correct answer is:
// Part 1: 53974
// Part 2: 52840
