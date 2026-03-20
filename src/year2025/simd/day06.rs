use std::simd::{Select, prelude::*};

use crate::util::parse::ParseUnsigned;

pub enum Op {
    Sum,
    Prod,
}

type Input<'a> = (Vec<&'a str>, Vec<Op>);

pub fn parse(input: &'_ str) -> Input<'_> {
    let num_lines = input.bytes().filter(|&b| b == b'\n').count() + 1;

    let mut lines = input.splitn(num_lines, '\n');

    let nums: Vec<&str> = lines.by_ref().take(num_lines - 1).collect();
    let ops = lines
        .next()
        .unwrap()
        .bytes()
        .filter_map(|b| match b {
            b'+' => Some(Op::Sum),
            b'*' => Some(Op::Prod),
            _ => None,
        })
        .collect();

    (nums, ops)
}

pub fn part1((nums, ops): &Input) -> u64 {
    let mut results = vec![0; ops.len()];

    for row in nums {
        results
            .iter_mut()
            .zip(ParseUnsigned::<u64>::new(row.bytes()))
            .zip(ops.iter())
            .for_each(|((result, num), op)| match op {
                Op::Sum => *result += num,
                Op::Prod => {
                    if *result == 0 {
                        *result = num;
                    } else {
                        *result *= num;
                    }
                }
            })
    }

    results.into_iter().sum()
}

pub fn part2((nums, ops): &Input) -> u64 {
    const SIMD_LANE_WIDTH: usize = 8;
    type CharSimdType = Simd<u8, SIMD_LANE_WIDTH>;
    type IntSimdType = Simd<u16, SIMD_LANE_WIDTH>;

    const TO_INT: CharSimdType = CharSimdType::splat(0x0F);
    const SPACES: CharSimdType = CharSimdType::splat(b' ');
    const ONES: IntSimdType = IntSimdType::splat(1);
    const TENS: IntSimdType = IntSimdType::splat(10);

    let line_len = nums[0].len();

    let mut op_iter = ops.iter();
    let mut current_op = op_iter.next().unwrap();

    let mut res = 0;

    let mut i = 0;

    let mut current_sum_val = 0;
    let mut current_prod_val = 1;

    while i + SIMD_LANE_WIDTH <= line_len {
        let mut simd_total = IntSimdType::splat(0);

        for &num_row in nums {
            let chunk = CharSimdType::from_slice(&num_row.as_bytes()[i..]);
            let chunk_val = chunk & TO_INT;
            let empty_mask = chunk.simd_eq(SPACES);
            let int_chunk: IntSimdType = chunk_val.cast();

            simd_total *= empty_mask.select(ONES, TENS);
            simd_total += int_chunk;
        }

        for j in 0..SIMD_LANE_WIDTH {
            if simd_total[j] != 0 {
                current_sum_val += simd_total[j] as u64;
                current_prod_val *= simd_total[j] as u64;
            } else {
                res += match current_op {
                    Op::Sum => current_sum_val,
                    Op::Prod => current_prod_val,
                };

                current_sum_val = 0;
                current_prod_val = 1;
                current_op = op_iter.next().unwrap();
            }
        }

        i += SIMD_LANE_WIDTH;
    }

    while i < line_len {
        let mut tmp = 0;

        for &num_row in nums {
            let byte = num_row.as_bytes()[i] & 0x0F;
            tmp *= (10 * (byte != 0) as u8 + (byte == 0) as u8) as u64;
            tmp += byte as u64;
        }

        if tmp != 0 {
            current_sum_val += tmp as u64;
            current_prod_val *= tmp as u64;
        } else {
            res += match current_op {
                Op::Sum => current_sum_val,
                Op::Prod => current_prod_val,
            };

            current_op = op_iter.next().unwrap();
            current_sum_val = 0;
            current_prod_val = 1;
        }

        i += 1;
    }

    res += match current_op {
        Op::Sum => current_sum_val,
        Op::Prod => current_prod_val,
    };

    res
}

// Answers for my input:
// Part 1: 6209956042374
// Part 2: 12608160008022
