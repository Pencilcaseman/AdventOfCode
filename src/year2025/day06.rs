use std::simd::prelude::*;

type Input<'a> = (&'a [u8], usize);

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

pub fn part1(&(bytes, line_len): &Input) -> u64 {
    type SimdType = u8x64;
    const MAX_EQUATIONS: usize = 1024;

    let len = bytes.len();
    let num_number_lines = len / line_len - 1;

    let op_line = &bytes[num_number_lines * (line_len + 1)..];
    let mut operators = [0; MAX_EQUATIONS];
    let mut op_count = 0;

    for &b in op_line {
        if b == b'+' || b == b'*' {
            if op_count < MAX_EQUATIONS {
                operators[op_count] = b;
                op_count += 1;
            }
        }
    }

    let mut results = [0; MAX_EQUATIONS];
    let mut equation_idx;

    let space_splat = SimdType::splat(b' ');

    let mut line_idx = 0;

    while line_idx < num_number_lines {
        let line = &bytes[line_idx * (line_len + 1)..];

        let mut start = 0;
        let mut end;
        let mut prev_ended_on_0 = false;
        equation_idx = 0;

        let mut i = 0;
        while i + SimdType::LEN < line_len {
            let chunk = SimdType::from_slice(&line[i..]);
            let mut mask = chunk.simd_eq(space_splat).to_bitmask();

            if mask & 1 != 0 {
                if prev_ended_on_0 {
                    // Previous chunk ended on a digit, so we must parse it

                    end = i;

                    let num =
                        atoi_simd::parse_pos::<u16, false>(&line[start..end])
                            .unwrap() as u64;

                    if operators[equation_idx] == b'+' {
                        results[equation_idx] += num;
                    } else if operators[equation_idx] == b'*' {
                        if results[equation_idx] == 0 {
                            results[equation_idx] = num;
                        } else {
                            results[equation_idx] *= num;
                        }
                    }
                    equation_idx += 1;

                    start = end;
                }

                // Clear to first zero
                let trailing = mask.trailing_ones();

                mask ^= (1 << trailing) - 1;
                start = start + trailing as usize;
            }

            prev_ended_on_0 = mask & (1 << SimdType::LEN - 1) == 0;

            while mask != 0 {
                let trailing = mask.trailing_zeros();
                end = i + trailing as usize;

                let num = atoi_simd::parse_pos::<u16, false>(&line[start..end])
                    .unwrap() as u64;

                if operators[equation_idx] == b'+' {
                    results[equation_idx] += num;
                } else if operators[equation_idx] == b'*' {
                    if results[equation_idx] == 0 {
                        results[equation_idx] = num;
                    } else {
                        results[equation_idx] *= num;
                    }
                }
                equation_idx += 1;

                // Unset first bit
                mask ^= 1 << trailing;

                // Unset the rest of the 1 bits in this block of spaces.
                // This allows us to handle multiple spaces (somewhat)
                // efficiently

                let trailing_ones =
                    (mask.unbounded_shr(trailing + 1)).trailing_ones();
                mask ^= (1u64.unbounded_shl(trailing_ones) - 1)
                    .unbounded_shl(trailing + 1);
                start = end + trailing_ones as usize + 1;
            }

            i += SimdType::LEN;
        }

        // Cases
        // - Chunk found start but not end
        // - Chunk found end

        if line[i] == b' ' {
            if prev_ended_on_0 {
                // Previous chunk ended on a digit, so we must parse it

                end = i;

                let num = atoi_simd::parse_pos::<u16, false>(&line[start..end])
                    .unwrap() as u64;

                if operators[equation_idx] == b'+' {
                    results[equation_idx] += num;
                } else if operators[equation_idx] == b'*' {
                    if results[equation_idx] == 0 {
                        results[equation_idx] = num;
                    } else {
                        results[equation_idx] *= num;
                    }
                }
                equation_idx += 1;

                start = end + 1;
            }
        }

        while start < line_len {
            // Skip spaces
            while line[start] == b' ' {
                start += 1;
            }

            let mut end = start + 1;
            while line[end] != b' ' && line[end] != b'\n' {
                end += 1;
            }

            let num = atoi_simd::parse_pos::<u16, false>(&line[start..end])
                .unwrap() as u64;

            if operators[equation_idx] == b'+' {
                results[equation_idx] += num;
            } else if operators[equation_idx] == b'*' {
                if results[equation_idx] == 0 {
                    results[equation_idx] = num;
                } else {
                    results[equation_idx] *= num;
                }
            }
            equation_idx += 1;

            start = end + 1;
        }

        line_idx += 1;
    }

    results.into_iter().sum()
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
