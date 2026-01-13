use std::simd::{
    Simd,
    cmp::{SimdPartialEq, SimdPartialOrd},
    u8x16,
};

type Input<'a> = &'a str;

pub fn parse<'a>(input: &'a str) -> Input<'a> {
    input
}

pub fn part1(input: &Input) -> u64 {
    let bytes = input.as_bytes();
    let len = bytes.len();

    let mut num1;
    let mut num2;

    let mut res = 0;

    let mut start = 0;
    let mut i = 0;

    let newline_splat = Simd::splat(b'\n');

    while i + 16 < len {
        let tmp_bytes = u8x16::from_slice(&bytes[i..]);
        let mut mask = tmp_bytes.simd_eq(newline_splat).to_bitmask();

        while mask != 0 {
            let trailing = mask.trailing_zeros();
            let end = i + trailing as usize;

            let copy_from = end - 2;

            num1 = bytes[end - 2];
            num2 = bytes[end - 1];

            let mut j = copy_from - start - 1;

            while j > 16 {
                let gatekeeper = Simd::splat(num1);
                let chunk = u8x16::from_slice(&bytes[start + j - 15..]);
                let mut gate_kept = chunk.simd_ge(gatekeeper).to_bitmask();

                while gate_kept != 0 {
                    let leading = (gate_kept as u16).leading_zeros();
                    let idx = 15 - leading;

                    let val = chunk[idx as usize];
                    if val >= num1 {
                        num2 = num2.max(num1);
                        num1 = val;
                    }

                    gate_kept ^= 1 << idx;
                }

                j -= 16;
            }

            while j < usize::MAX {
                let tmp = bytes[start + j];

                if tmp >= num1 {
                    num2 = num2.max(num1);
                    num1 = tmp;
                }

                j = j.wrapping_sub(1);
            }

            res += ((num1 & 0x0F) * 10 + (num2 & 0x0F)) as u64;

            start = end + 1;
            mask ^= 1 << trailing;
        }

        i += 16;
    }

    while i <= len {
        if i == len || bytes[i] == b'\n' {
            let end = i;

            let copy_from = end - 2;

            num1 = bytes[end - 2];
            num2 = bytes[end - 1];

            let mut j = copy_from - start - 1;

            while j < usize::MAX {
                let tmp = bytes[start + j];

                if tmp >= num1 {
                    num2 = num2.max(num1);
                    num1 = tmp;
                }

                j = j.wrapping_sub(1);
            }

            res += ((num1 & 0x0F) * 10 + (num2 & 0x0F)) as u64;

            start = end + 1;
        }

        i += 1;
    }

    res
}

pub fn part2(input: &Input) -> u64 {
    solve::<12>(input)
}

pub fn solve<const N: usize>(input: &Input) -> u64 {
    let bytes = input.as_bytes();
    let len = bytes.len();

    let mut num = [0u8; N];

    let mut res = 0;

    let mut start = 0;
    let mut i = 0;

    let newline_splat = Simd::splat(b'\n');

    while i + 16 < len {
        let tmp_bytes = u8x16::from_slice(&bytes[i..]);
        let mut mask = tmp_bytes.simd_eq(newline_splat).to_bitmask();

        while mask != 0 {
            let trailing = mask.trailing_zeros();
            let end = i + trailing as usize;

            let copy_from = end - N;
            num.copy_from_slice(&bytes[copy_from..copy_from + N]);

            let mut j = copy_from - start - 1;

            while j > 16 {
                let gatekeeper = Simd::splat(num[0]);
                let chunk = u8x16::from_slice(&bytes[start + j - 15..]);
                let gate_kept = chunk.simd_ge(gatekeeper).to_bitmask();

                // Can skip entire chunk
                if gate_kept == 0 {
                    j -= 16;
                    continue;
                }

                for k in (0..16).rev() {
                    if gate_kept & (1 << k) != 0 {
                        let mut tmp = chunk[k];

                        for n in &mut num {
                            if tmp < *n {
                                break;
                            }
                            tmp = std::mem::replace(n, tmp);
                        }
                    }
                }

                j -= 16;
            }

            while j < usize::MAX {
                let mut tmp = bytes[start + j];

                for k in &mut num {
                    if tmp < *k {
                        break;
                    }
                    tmp = std::mem::replace(k, tmp);
                }

                j = j.wrapping_sub(1);
            }

            let mut tmp_res = 0;
            for n in &num {
                tmp_res = tmp_res * 10 + (n & 0x0F) as u64;
            }

            res += tmp_res;

            start = end + 1;
            mask ^= 1 << trailing;
        }

        i += 16;
    }

    while i <= len {
        if i == len || bytes[i] == b'\n' {
            let end = i;

            let copy_from = end - N;
            num.copy_from_slice(&bytes[copy_from..copy_from + N]);

            let mut j = copy_from - start - 1;

            while j < usize::MAX {
                let mut tmp = bytes[start + j];

                for n in &mut num {
                    if tmp < *n {
                        break;
                    }
                    tmp = std::mem::replace(n, tmp);
                }

                j = j.wrapping_sub(1);
            }

            let mut tmp_res = 0;
            for n in &num {
                tmp_res = tmp_res * 10 + (n & 0x0F) as u64;
            }

            res += tmp_res;

            start = end + 1;
        }

        i += 1;
    }

    res
}

// Answers for my input:
// Part 1: 17263
// Part 2: 170731717900423
