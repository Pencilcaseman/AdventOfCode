use std::simd::{Select, prelude::*};

use crate::util::parse::try_parse_unsigned;

pub enum Op {
    Sum,
    Prod,
}

type Input = (u64, u64);

pub fn parse(input: &str) -> Input {
    const SIMD_LANE_WIDTH: usize = 16;
    type CharSimdType = Simd<u8, SIMD_LANE_WIDTH>;
    type IntSimdType = Simd<u16, SIMD_LANE_WIDTH>;

    const TO_INT: CharSimdType = CharSimdType::splat(0x0F);
    const SPACES: CharSimdType = CharSimdType::splat(b' ');
    const ZEROS: IntSimdType = IntSimdType::splat(0);
    const ONES: IntSimdType = IntSimdType::splat(1);
    const TENS: IntSimdType = IntSimdType::splat(10);

    let lines: Vec<&str> = input.lines().collect();
    let nums = &lines[0..lines.len() - 1];

    let mut op_iter = lines[lines.len() - 1].bytes().filter_map(|b| match b {
        b'+' => Some(Op::Sum),
        b'*' => Some(Op::Prod),
        _ => None,
    });

    let mut current_op = op_iter.next().unwrap();
    let line_len = nums[0].len();

    let mut p1: u64 = 0;
    let mut p2: u64 = 0;

    let mut h_sum: u64 = 0;
    let mut h_prod: u64 = 1;
    let mut eq_start = 0;

    let mut v_sum: u64 = 0;
    let mut v_prod: u64 = 1;

    let mut i = 0;
    while i + SIMD_LANE_WIDTH <= line_len {
        let mut simd_total = IntSimdType::splat(0);

        for num_row in nums {
            let chunk = CharSimdType::from_slice(&num_row.as_bytes()[i..]);
            let chunk_val = chunk & TO_INT;
            let empty_mask = chunk.simd_eq(SPACES);
            let int_chunk: IntSimdType = chunk_val.cast();

            simd_total *= empty_mask.select(ONES, TENS);
            simd_total += int_chunk;
        }

        let arr = simd_total.to_array();
        let mut sep_mask = simd_total.simd_eq(ZEROS).to_bitmask();
        let mut chunk_pos = 0;

        while sep_mask != 0 {
            let tz = sep_mask.trailing_zeros() as usize;

            for k in chunk_pos..tz {
                v_sum += arr[k] as u64;
                v_prod *= arr[k] as u64;
            }

            let eq_end = i + tz;

            for num_row in nums {
                if let Some(num) = try_parse_unsigned::<u64>(
                    &num_row.as_bytes()[eq_start..eq_end],
                ) {
                    h_sum += num;
                    h_prod *= num;
                }
            }

            eq_start = eq_end;

            match current_op {
                Op::Sum => {
                    p1 += h_sum;
                    p2 += v_sum;
                }
                Op::Prod => {
                    p1 += h_prod;
                    p2 += v_prod;
                }
            }

            h_sum = 0;
            h_prod = 1;
            v_sum = 0;
            v_prod = 1;
            current_op = op_iter.next().unwrap();

            sep_mask &= sep_mask - 1;
            chunk_pos = tz + 1;
        }

        for k in chunk_pos..SIMD_LANE_WIDTH {
            v_sum += arr[k] as u64;
            v_prod *= arr[k] as u64;
        }

        i += SIMD_LANE_WIDTH;
    }

    while i < line_len {
        let mut tmp: u64 = 0;

        for num_row in nums {
            let byte = num_row.as_bytes()[i] & 0x0F;
            tmp *= (10 * (byte != 0) as u8 + (byte == 0) as u8) as u64;
            tmp += byte as u64;
        }

        if tmp != 0 {
            v_sum += tmp;
            v_prod *= tmp;
        } else {
            let eq_end = i;

            for num_row in nums {
                if let Some(num) = try_parse_unsigned::<u64>(
                    &num_row.as_bytes()[eq_start..eq_end],
                ) {
                    h_sum += num;
                    h_prod *= num;
                }
            }

            eq_start = eq_end;

            match current_op {
                Op::Sum => {
                    p1 += h_sum;
                    p2 += v_sum;
                }
                Op::Prod => {
                    p1 += h_prod;
                    p2 += v_prod;
                }
            }

            h_sum = 0;
            h_prod = 1;
            v_sum = 0;
            v_prod = 1;
            current_op = op_iter.next().unwrap();
        }

        i += 1;
    }

    let eq_end = line_len;
    for num_row in nums {
        if let Some(num) =
            try_parse_unsigned::<u64>(&num_row.as_bytes()[eq_start..eq_end])
        {
            h_sum += num;
            h_prod *= num;
        }
    }

    p1 += match current_op {
        Op::Sum => h_sum,
        Op::Prod => h_prod,
    };

    p2 += match current_op {
        Op::Sum => v_sum,
        Op::Prod => v_prod,
    };

    (p1, p2)
}

pub fn part1(input: &Input) -> u64 {
    input.0
}

pub fn part2(input: &Input) -> u64 {
    input.1
}

// Answers for my input:
// Part 1: 6209956042374
// Part 2: 12608160008022
