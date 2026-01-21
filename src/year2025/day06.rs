use std::{array::from_fn, hint::unreachable_unchecked, simd::prelude::*};

use crate::util::parse::ParseUnsigned;

pub enum Op {
    Sum,
    Prod,
}

// type Input<'a> = (Vec<&'a str>, Vec<Op>);
type Input<'a> = (&'a [u8], usize);

// pub fn parse(input: &'_ str) -> Input<'_> {
//     let num_lines = input.bytes().filter(|&b| b == b'\n').count() + 1;
//
//     let mut lines = input.splitn(num_lines, '\n');
//
//     let nums: Vec<&str> = lines.by_ref().take(num_lines - 1).collect();
//     let ops = lines
//         .next()
//         .unwrap()
//         .bytes()
//         .filter_map(|b| match b {
//             b'+' => Some(Op::Sum),
//             b'*' => Some(Op::Prod),
//             _ => None,
//         })
//         .collect();
//
//     (nums, ops)
// }

pub fn parse(input: &'_ str) -> Input<'_> {
    type SimdType = u8x64;

    let bytes = input.as_bytes();
    let len = bytes.len();

    let newline_splat = SimdType::splat(b'\n');
    let mut i = 0;

    while i + SimdType::LEN < len {
        let chunk = SimdType::from_slice(&bytes[i..]);
        let mask = chunk.simd_eq(newline_splat).to_bitmask();

        if mask != 0 {
            return (bytes, i + mask.trailing_zeros() as usize);
        }

        i += SimdType::LEN;
    }

    while i < len {
        if bytes[i] == b'\n' {
            return (bytes, i);
        }

        i += 1;
    }

    panic!("No newline found");
}

// pub fn part1((nums, ops): &Input) -> u64 {
//     let mut results = vec![0; ops.len()];
//
//     for row in nums {
//         results
//             .iter_mut()
//             .zip(ParseUnsigned::<u64>::new(row.bytes()))
//             .zip(ops.iter())
//             .for_each(|((result, num), op)| match op {
//                 Op::Sum => *result += num,
//                 Op::Prod => {
//                     if *result == 0 {
//                         *result = num;
//                     } else {
//                         *result *= num;
//                     }
//                 }
//             })
//     }
//
//     results.into_iter().sum()
// }

pub fn part1((bytes, line_len): &Input) -> u64 {
    type SimdType = u8x64;
    const MAX_NUMS_PER_EQN: usize = 4;

    let mut total = 0;

    let num_lines = bytes.len() / line_len;
    let lines_with_nums = num_lines - 1;
    let ptr = bytes.as_ptr();

    debug_assert!(
        lines_with_nums <= MAX_NUMS_PER_EQN,
        "Too many lines in equation. Increase MAX_NUMS_PER_EQN"
    );

    // Safety: At most num_lines are valid pointers, but if we never dereference
    // pointers after that point, this is not unsafe.
    let line_pointers: [_; MAX_NUMS_PER_EQN] =
        from_fn(|i| unsafe { ptr.add(i * (line_len + 1)) });

    let op_pointer = unsafe { ptr.add((line_len + 1) * lines_with_nums) };

    let mut nums = [0; MAX_NUMS_PER_EQN];
    let mut i = 0;

    // '0' => 00110000
    // '1' => 00110001
    // '2' => 00110010
    // '3' => 00110011
    // '4' => 00110100
    // '5' => 00110101
    // '6' => 00110110
    // '7' => 00110111
    // '8' => 00111000
    // '9' => 00111001
    // ' ' => 00100000
    //
    // 0x0F => 00001111

    // let hex_mask_splat = SimdType::splat(0x0F);
    let space_splat = SimdType::splat(b' ');

    let mut start = 0;

    while i + SimdType::LEN < *line_len {
        let mut mask = u64::MAX;

        (0..lines_with_nums).for_each(|j| {
            let chunk = unsafe {
                line_pointers[j].add(i).cast::<SimdType>().read_unaligned()
            };
            mask &= chunk.simd_eq(space_splat).to_bitmask();
        });

        while mask != 0 {
            let trailing = mask.trailing_zeros() as usize;
            let end = i + trailing;

            for k in 0..lines_with_nums {
                for j in start..end {
                    let val = unsafe { *line_pointers[k].add(j) };

                    nums[k] = ((val != b' ') as u64 * 9 + 1) * nums[k]
                        + (val & 0x0F) as u64;
                }
            }

            let op = unsafe { *op_pointer.add(start) };
            total += match op {
                b'+' => nums[..lines_with_nums].iter().sum::<u64>(),
                b'*' => nums[..lines_with_nums].iter().product::<u64>(),
                _ => unsafe { unreachable_unchecked() },
            };

            nums.fill_with(Default::default);

            mask ^= 1 << trailing;
            start = end + 1;
        }

        i += SimdType::LEN;
    }

    while i <= *line_len {
        let all_empty = i == *line_len
            || (0..lines_with_nums)
                .all(|j| unsafe { *line_pointers[j].add(i) } == b' ');

        if all_empty {
            let end = i;

            for k in 0..lines_with_nums {
                for j in start..end {
                    let val = unsafe { *line_pointers[k].add(j) };

                    nums[k] = ((val != b' ') as u64 * 9 + 1) * nums[k]
                        + (val & 0x0F) as u64;
                }
            }

            let op = unsafe { *op_pointer.add(start) };
            total += match op {
                b'+' => nums[..lines_with_nums].iter().sum::<u64>(),
                b'*' => nums[..lines_with_nums].iter().product::<u64>(),
                _ => unreachable!(),
            };

            nums.fill_with(Default::default);

            start = end + 1;
        }

        i += 1;
    }

    total
}

// pub fn part2((nums, ops): &Input) -> u64 {
//     let mut vertical_nums = vec![0; nums[0].len()];
//
//     for line in nums {
//         for (vert, byte) in vertical_nums.iter_mut().zip(line.bytes()) {
//             if byte.wrapping_sub(b'0') < 10 {
//                 *vert = *vert * 10 + (byte - b'0') as u64;
//             }
//         }
//     }
//
//     ops.iter()
//         .zip(vertical_nums.split(|&n| n == 0))
//         .map(|(op, vert)| match op {
//             Op::Sum => vert.iter().sum::<u64>(),
//             Op::Prod => vert.iter().product(),
//         })
//         .sum()
// }

pub fn part2(input: &Input) -> u64 {
    0
}

// Answers for my input:
// Part 1: 6209956042374
// Part 2: 12608160008022
