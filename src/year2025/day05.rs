use itertools::Itertools;

use crate::util::parse::ParseUnsigned;

type Range = std::ops::Range<u64>;

type Input = (Vec<Range>, Vec<u64>);

pub fn parse(input: &str) -> Input {
    let (ranges, nums) = input.split_once("\n\n").unwrap();

    let mut ranges: Vec<(u64, u64)> =
        ParseUnsigned::new(ranges.bytes()).tuples().collect();
    let mut nums: Vec<u64> = ParseUnsigned::new(nums.bytes()).collect();

    ranges.sort_unstable();
    nums.sort_unstable();

    let mut range = 0..0;
    let mut merged = Vec::new();

    for (start, end) in ranges {
        if range.end > start {
            range.end = range.end.max(end + 1);
        } else {
            merged.push(range);
            range = start..(end + 1);
        }
    }
    merged.push(range);

    (merged, nums)
}

pub fn part1((ranges, nums): &Input) -> usize {
    ranges
        .iter()
        .map(|r| {
            let start = nums.binary_search(&r.start).unwrap_or_else(|e| e);
            let end = nums.binary_search(&r.end).unwrap_or_else(|e| e);

            end - start
        })
        .sum()
}

pub fn part2((ranges, _): &Input) -> u64 {
    ranges.iter().map(|r| r.end - r.start).sum()
}

// Answers for my input:
// Part 1: 773
// Part 2: 332067203034711
