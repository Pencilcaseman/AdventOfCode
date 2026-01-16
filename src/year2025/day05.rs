use std::simd::prelude::*;

type Input = (Vec<u64>, Vec<u64>, Vec<u64>);

pub fn parse(input: &str) -> Input {
    type SimdType = u8x64;

    let bytes = input.as_bytes();
    let len = bytes.len();

    let mut ranges = Vec::new();
    let mut nums = Vec::new();

    let mut start = 0;

    let mut i = 0;

    let newline_mask = Simd::splat(b'\n');

    let mut prev_ended_newline = false;

    'ranges_loop: loop {
        let chunk = SimdType::from_slice(&bytes[i..]);
        let original_mask = chunk.simd_eq(newline_mask).to_bitmask();
        let mut mask = original_mask;

        // Unlikely edge case where double newline spans two chunks
        if prev_ended_newline && (mask & 1) != 0 {
            i += 1;
            break 'ranges_loop;
        }

        while mask != 0 {
            let trailing = mask.trailing_zeros() as usize;
            let end = i + trailing;

            let (num1, count) =
                atoi_simd::parse_prefix_pos::<u64, true>(&bytes[start..])
                    .unwrap();
            let (num2, _) = atoi_simd::parse_prefix_pos::<u64, true>(
                &bytes[start + count + 1..],
            )
            .unwrap();

            ranges.push((num1, num2));

            start = end + 1;

            mask ^= 1 << trailing;

            if mask & (1 << (trailing + 1)) != 0 {
                // Adjacent newlines found. Stop parsing ranges.
                i += trailing + 2;
                break 'ranges_loop;
            }
        }

        prev_ended_newline = original_mask & (1 << (SimdType::LEN - 1)) != 0;

        i += SimdType::LEN;
    }

    while i < len
        && let Ok((num, count)) =
            atoi_simd::parse_prefix_pos::<u64, true>(&bytes[i..])
    {
        nums.push(num);
        i += count + 1;
    }

    ranges.sort_unstable();
    nums.sort_unstable();

    let mut range = 0..0;
    let mut merged_start = Vec::new();
    let mut merged_end = Vec::new();

    for (start, end) in ranges {
        if range.end >= start {
            range.end = range.end.max(end + 1);
        } else {
            merged_start.push(range.start);
            merged_end.push(range.end);
            range = start..(end + 1);
        }
    }

    merged_start.push(range.start);
    merged_end.push(range.end);

    while !merged_start.len().is_multiple_of(SimdType::LEN) {
        merged_start.push(0);
        merged_end.push(0);
    }

    (merged_start, merged_end, nums)
}

pub fn part1((ranges_start, ranges_end, nums): &Input) -> usize {
    let mut count = 0;
    let mut num_idx = 0;
    let nums_len = nums.len();

    for (&start, &end) in ranges_start.iter().zip(ranges_end) {
        while num_idx < nums_len && nums[num_idx] < start {
            num_idx += 1;
        }

        let saved_idx = num_idx;
        while num_idx < nums_len && nums[num_idx] < end {
            num_idx += 1;
        }

        count += num_idx - saved_idx;
    }

    count
}

pub fn part2((ranges_start, ranges_end, _): &Input) -> u64 {
    type SimdType = u64x8;

    debug_assert!(ranges_start.len().is_multiple_of(SimdType::LEN));

    let len = ranges_start.len();
    let mut acc = Simd::splat(0);

    let mut i = 0;
    while i < len {
        let chunk_start = SimdType::from_slice(&ranges_start[i..]);
        let chunk_end = SimdType::from_slice(&ranges_end[i..]);

        let diff = chunk_end - chunk_start;
        acc += diff;

        i += SimdType::LEN;
    }

    acc.reduce_sum()
}

// Answers for my input:
// Part 1: 773
// Part 2: 332067203034711
