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

    // 24.7689

    const TO_INT: CharSimdType = CharSimdType::splat(0x0F);
    const SPACES: CharSimdType = CharSimdType::splat(b' ');
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

    let mut p1 = 0;
    let mut p2 = 0;

    let mut horizontal_sum_val = 0;
    let mut horizontal_prod_val = 1;

    // For part 1
    let mut eq_start = 0;

    // For part 2
    let mut vertical_sum_val = 0;
    let mut vertical_prod_val = 1;

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

        for j in 0..SIMD_LANE_WIDTH {
            if simd_total[j] != 0 {
                vertical_sum_val += simd_total[j] as u64;
                vertical_prod_val *= simd_total[j] as u64;
            } else {
                let eq_end = i + j;

                for num_row in nums {
                    if let Some(num) = try_parse_unsigned::<u64>(
                        &num_row.as_bytes()[eq_start..eq_end],
                    ) {
                        horizontal_sum_val += num;
                        horizontal_prod_val *= num;
                    }
                }

                eq_start = eq_end;

                match current_op {
                    Op::Sum => {
                        p1 += horizontal_sum_val as u64;
                        p2 += vertical_sum_val as u64;
                    }
                    Op::Prod => {
                        p1 += horizontal_prod_val as u64;
                        p2 += vertical_prod_val as u64;
                    }
                }

                horizontal_sum_val = 0;
                horizontal_prod_val = 1;

                vertical_sum_val = 0;
                vertical_prod_val = 1;
                current_op = op_iter.next().unwrap();
            }
        }

        i += SIMD_LANE_WIDTH;
    }

    while i < line_len {
        let mut tmp = 0;

        for num_row in nums {
            let byte = num_row.as_bytes()[i] & 0x0F;
            tmp *= (10 * (byte != 0) as u8 + (byte == 0) as u8) as u64;
            tmp += byte as u64;
        }

        if tmp != 0 {
            vertical_sum_val += tmp as u64;
            vertical_prod_val *= tmp as u64;
        } else {
            let eq_end = i;

            for num_row in nums {
                if let Some(num) = try_parse_unsigned::<u64>(
                    &num_row.as_bytes()[eq_start..eq_end],
                ) {
                    horizontal_sum_val += num;
                    horizontal_prod_val *= num;
                }
            }

            eq_start = eq_end;

            match current_op {
                Op::Sum => {
                    p1 += horizontal_sum_val;
                    p2 += vertical_sum_val;
                }
                Op::Prod => {
                    p1 += horizontal_prod_val;
                    p2 += vertical_prod_val;
                }
            }

            horizontal_sum_val = 0;
            horizontal_prod_val = 1;

            vertical_sum_val = 0;
            vertical_prod_val = 1;
            current_op = op_iter.next().unwrap();
        }

        i += 1;
    }

    let eq_end = line_len;
    for num_row in nums {
        if let Some(num) =
            try_parse_unsigned::<u64>(&num_row.as_bytes()[eq_start..eq_end])
        {
            horizontal_sum_val += num;
            horizontal_prod_val *= num;
        }
    }

    p1 += match current_op {
        Op::Sum => horizontal_sum_val,
        Op::Prod => horizontal_prod_val,
    };

    p2 += match current_op {
        Op::Sum => vertical_sum_val,
        Op::Prod => vertical_prod_val,
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
