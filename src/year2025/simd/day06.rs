use std::simd::{Select, prelude::*};

use crate::util::parse::try_parse_unsigned;

pub enum Op {
    Sum,
    Prod,
}

type Input = (u64, u64);

pub fn parse_old(input: &str) -> Input {
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

pub fn parse(input: &str) -> Input {
    const SIMD_LANE_WIDTH: usize = 32;
    type CharSimdType = Simd<u8, SIMD_LANE_WIDTH>;
    type IntSimdType = Simd<u16, SIMD_LANE_WIDTH>;

    // 24.7689

    const TO_INT: CharSimdType = CharSimdType::splat(0x0F);
    const SPACES: CharSimdType = CharSimdType::splat(b' ');
    const ZEROS: IntSimdType = IntSimdType::splat(0);
    const ONES: IntSimdType = IntSimdType::splat(1);
    const TENS: IntSimdType = IntSimdType::splat(10);

    let lines: Vec<&str> = input.lines().collect();
    let nums = &lines[0..lines.len() - 1];

    let line_len = nums[0].len();

    let mut vertical_nums = vec![0u16; line_len]; // Known size
    let mut horizontal_nums = Vec::new();

    let mut section_start = 0;

    let mut i = 0;
    while i + SIMD_LANE_WIDTH <= line_len {
        let mut simd_vert = IntSimdType::splat(0);

        for num_row in nums {
            let chunk = CharSimdType::from_slice(&num_row.as_bytes()[i..]);
            let chunk_val = chunk & TO_INT;
            let empty_mask = chunk.simd_eq(SPACES);
            let int_chunk: IntSimdType = chunk_val.cast();

            simd_vert *= empty_mask.select(ONES, TENS);
            simd_vert += int_chunk;
        }

        vertical_nums[i..i + SIMD_LANE_WIDTH]
            .copy_from_slice(&simd_vert.to_array());

        let mut section_mask = simd_vert.simd_eq(ZEROS).to_bitmask();

        while section_mask != 0 {
            let trailing = section_mask.trailing_zeros();
            let section_end = i + trailing as usize;

            for num_row in nums {
                if let Some(num) = try_parse_unsigned::<u16>(
                    &num_row.as_bytes()[section_start..section_end],
                ) {
                    horizontal_nums.push(num);
                }
            }

            section_mask ^= 1 << trailing;
            section_start = section_end;
        }

        i += SIMD_LANE_WIDTH;
    }

    while i < line_len {
        let mut vert = 0;

        for num_row in nums {
            let byte = num_row.as_bytes()[i] & 0x0F;
            vert *= (10 * (byte != 0) as u8 + (byte == 0) as u8) as u16;
            vert += byte as u16;
        }

        vertical_nums[i] = vert;

        if vert == 0 {
            let section_end = i;

            for num_row in nums {
                if let Some(num) = try_parse_unsigned::<u16>(
                    &num_row.as_bytes()[section_start..section_end],
                ) {
                    horizontal_nums.push(num);
                }
            }

            section_start = section_end;
        }

        i += 1;
    }

    let section_end = line_len;
    for num_row in nums {
        if let Some(num) = try_parse_unsigned::<u16>(
            &num_row.as_bytes()[section_start..section_end],
        ) {
            horizontal_nums.push(num);
        }
    }

    let mut h_idx = 0;
    let mut v_idx = 0;

    let mut h_result = 0;
    let mut v_result = 0;

    for op in lines[lines.len() - 1].bytes().filter_map(|b| match b {
        b'+' => Some(Op::Sum),
        b'*' => Some(Op::Prod),
        _ => None,
    }) {
        let mut v_sum = 0;
        let mut v_prod = 1;

        while v_idx < vertical_nums.len() && vertical_nums[v_idx] != 0 {
            v_sum += vertical_nums[v_idx] as u64;
            v_prod *= vertical_nums[v_idx] as u64;
            v_idx += 1;
        }

        let mut h_sum = 0;
        let mut h_prod = 1;

        for i in 0..nums.len() {
            h_sum += horizontal_nums[h_idx + i] as u64;
            h_prod *= horizontal_nums[h_idx + i] as u64;
        }

        match op {
            Op::Sum => {
                h_result += h_sum;
                v_result += v_sum;
            }
            Op::Prod => {
                h_result += h_prod;
                v_result += v_prod;
            }
        }

        v_idx += 1;
        h_idx += nums.len();
    }

    (h_result, v_result)
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
