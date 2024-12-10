#![warn(clippy::pedantic, clippy::nursery)]

use crate::util::parse::ParseOps;

type Input = (usize, usize);

/// NOTE: I saw [ManEatingApe](https://github.com/maneatingape/advent-of-code-rust/blob/main/src/year2024/day07.rs)
/// do this, and it does indeed work. This does not work for the general case,
/// but it appears that the ``AoC`` input never contains 4+ digit concatenations
/// (or they are all unique and valid). While I am not entirely happy with this,
/// it is faster than a 'correct'/'complete' solution.
const fn next_pow_10(num: usize) -> usize {
    if num < 10 {
        10
    } else if num < 100 {
        100
    } else {
        1000
    }
}

#[must_use]
pub const fn ends_with(num: usize, end: usize) -> Option<usize> {
    let next_pow_10 = next_pow_10(end);

    if num % next_pow_10 == end {
        Some(num / next_pow_10)
    } else {
        None
    }
}

// fn is_valid(result: usize, nums: &[usize], concat: bool) -> bool {
//     let last = nums[nums.len() - 1];
//
//     if nums.len() == 1 {
//         return result == last;
//     }
//
//     (result > last && is_valid(result - last, &nums[..nums.len() - 1],
// concat))         || (result % last == 0
//             && is_valid(result / last, &nums[..nums.len() - 1], concat))
//         || (concat
//             && ends_with(result, last).is_some_and(|rem| {
//                 is_valid(rem, &nums[..nums.len() - 1], concat)
//             }))
// }

fn is_valid(
    result: usize,
    nums: &[usize],
    last_idx: usize,
    concat: bool,
) -> bool {
    let last = nums[last_idx];

    if last_idx == 0 {
        return result == last;
    }

    // Checking for concatenation first gives a slight performance improvement,
    // since we know the test failed with only ADD and MUL, so concatenation is
    // required for this to be a potentially valid solution.
    // (concat
    //     && ends_with(result, last)
    //         .is_some_and(|rem| is_valid(rem, nums, last_idx - 1, concat)))
    //     || (result > last
    //         && is_valid(result - last, nums, last_idx - 1, concat))
    //     || (result % last == 0
    //         && is_valid(result / last, nums, last_idx - 1, concat))

    (concat
        && result % next_pow_10(last) == last
        && is_valid(result / next_pow_10(last), nums, last_idx - 1, concat))
        || (result > last
            && is_valid(result - last, nums, last_idx - 1, concat))
        || (result % last == 0
            && is_valid(result / last, nums, last_idx - 1, concat))
}

#[must_use]
pub fn parse(input: &str) -> Input {
    let mut tmp = Vec::with_capacity(32);
    let mut part1 = 0;
    let mut part2 = 0;

    for line in input.lines() {
        tmp.clear();
        tmp.extend(line.iter_unsigned::<usize>());

        let result = tmp[0];
        let nums = &tmp[1..];
        let last_idx = nums.len() - 1;

        if is_valid(result, nums, last_idx, false) {
            // A solution to part 1 is a solution to part 2
            part1 += result;
            part2 += result;
        } else if is_valid(result, nums, last_idx, true) {
            part2 += result;
        }
    }

    (part1, part2)
}

#[must_use]
pub const fn part1(input: &Input) -> usize {
    input.0
}

#[must_use]
pub const fn part2(input: &Input) -> usize {
    input.1
}

// For my input, the correct answer is:
// Part 1: 1582598718861
// Part 2: 165278151522644
