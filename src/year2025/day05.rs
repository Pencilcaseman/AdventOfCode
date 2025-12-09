use itertools::Itertools;

use crate::util::parse::ParseUnsigned;

type Input = (Vec<(u64, u64)>, Vec<u64>);

pub fn parse(input: &str) -> Input {
    let (ranges, nums) = input.split_once("\n\n").unwrap();

    let mut ranges: Vec<(u64, u64)> =
        ParseUnsigned::new(ranges.bytes()).tuples().collect();
    let mut nums: Vec<u64> = ParseUnsigned::new(nums.bytes()).collect();

    ranges.sort_unstable();
    nums.sort_unstable();

    let mut range = (0, 0);
    let mut merged = Vec::new();

    for (start, end) in ranges {
        if range.1 > start {
            range.1 = range.1.max(end + 1);
        } else {
            merged.push(range);
            range = (start, end + 1);
        }
    }
    merged.push(range);

    (merged, nums)
}

pub fn part1((ranges, nums): &Input) -> usize {
    nums.iter()
        .filter(|&n| {
            ranges
                .binary_search_by(|(start, end)| {
                    if n < start {
                        std::cmp::Ordering::Greater
                    } else if n > end {
                        std::cmp::Ordering::Less
                    } else {
                        std::cmp::Ordering::Equal
                    }
                })
                .is_ok()
        })
        .count()
}

pub fn part2((ranges, _): &Input) -> u64 {
    ranges.iter().map(|(start, end)| end - start).sum()
}

// Answers for my input:
// Part 1: 773
// Part 2: 332067203034711
