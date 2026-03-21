use std::simd::prelude::*;

type Input = (u32, u64);

pub fn parse(input: &str) -> Input {
    type SimdType = u8x64;

    let bytes = input.as_bytes();
    let total_len = bytes.len();
    let mut len = 0;

    const NEWLINE_SPLAT: SimdType = SimdType::splat(b'\n');

    let mut idx = 0;

    debug_assert!(total_len >= SimdType::LEN, "Scalar tail is not handled");

    while idx + SimdType::LEN <= total_len {
        let chunk = SimdType::from_slice(&bytes[idx..]);
        let mask = chunk.simd_eq(NEWLINE_SPLAT);
        let bitmask = mask.to_bitmask();

        if bitmask != 0 {
            len = idx + bitmask.trailing_zeros() as usize;
            break;
        }

        idx += SimdType::LEN;
    }

    let len = len;
    let stride = (len + 1) * 2;

    let mid = len / 2;

    let mut part1 = 0;

    let mut current = vec![0u64; len];

    current[mid] = 1;

    let possible_range = |row| ((mid - row)..(mid + row + 1)).step_by(2);

    idx = stride;
    let mut row = 0;

    while idx < total_len {
        for i in possible_range(row) {
            let tmp = current[i];

            if tmp > 0 && bytes[idx + i] == b'^' {
                part1 += 1;

                current[i] = 0;
                current[i - 1] += tmp;
                current[i + 1] += tmp;
            }
        }

        idx += stride;
        row += 1;
    }

    (part1, current.into_iter().sum())
}

pub fn part1(input: &Input) -> u32 {
    input.0
}

pub fn part2(input: &Input) -> u64 {
    input.1
}

// Answers for my input:
// Part 1: 1662
// Part 2: 40941112789504
