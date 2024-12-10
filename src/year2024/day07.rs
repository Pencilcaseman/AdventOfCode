#![warn(clippy::pedantic, clippy::nursery)]

use crate::util::parse::ParseOps;

type Input = Vec<Vec<usize>>;

#[must_use]
pub fn parse(input: &str) -> Input {
    input.lines().map(|line| line.iter_unsigned().collect()).collect()
}

#[must_use]
pub const fn ends_with(num: usize, end: usize) -> Option<usize> {
    if end == 0 {
        return Some(num / 10);
    }

    let next_pow_10 = 10usize.pow(end.ilog10() + 1);

    if num % next_pow_10 == end {
        Some(num / next_pow_10)
    } else {
        None
    }
}

fn valid_part1(result: usize, nums: &[usize]) -> bool {
    let Some(&last) = nums.last() else { return result == 0 };

    if nums.len() == 1 {
        return result == last;
    }

    if result > last && valid_part1(result - last, &nums[..nums.len() - 1]) {
        return true;
    }

    if result % last == 0 && valid_part1(result / last, &nums[..nums.len() - 1])
    {
        return true;
    }

    false
}

fn valid_part2(result: usize, nums: &[usize]) -> bool {
    let Some(&last) = nums.last() else { return result == 0 };

    if nums.len() == 1 {
        return result == last;
    }

    if result > last && valid_part2(result - last, &nums[..nums.len() - 1]) {
        return true;
    }

    if result % last == 0 && valid_part2(result / last, &nums[..nums.len() - 1])
    {
        return true;
    }

    if let Some(end) = ends_with(result, last) {
        if valid_part2(end, &nums[..nums.len() - 1]) {
            return true;
        }
    }

    false
}

#[must_use]
pub fn part1(input: &Input) -> usize {
    input
        .iter()
        .filter_map(|nums| {
            if valid_part1(nums[0], &nums[1..]) {
                Some(nums[0])
            } else {
                None
            }
        })
        .sum()
}

#[must_use]
pub fn part2(input: &Input) -> usize {
    input
        .iter()
        .filter_map(|nums| {
            if valid_part2(nums[0], &nums[1..]) {
                Some(nums[0])
            } else {
                None
            }
        })
        .sum()
}

// For my input, the correct answer is:
// Part 1: 1582598718861
// Part 2: 165278151522644
