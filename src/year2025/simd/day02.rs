use crate::util::integer::{num_length, pow10};

type Input = (u64, u64);

const DOUBLE_COUNT_REMOVAL: [i8; 12] = [0, 0, 1, 1, 0, 1, -1, 1, 0, 0, -1, 1];

use std::simd::{Simd, cmp::SimdPartialEq, u8x64};

pub fn parse(input: &str) -> Input {
    let bytes = input.as_bytes();
    let ptr = input.as_ptr();
    let len = bytes.len();

    let comma_splat = Simd::splat(b',');
    let hyphen_splat = Simd::splat(b'-');

    let mut i = 0;

    let mut p1 = 0;
    let mut p2 = 0;

    let mut start = 0;

    let mut num1 = 0;
    let mut num2 = 0;

    while i + 64 < len {
        let register = u8x64::from_slice(&bytes[i..]);
        let mut mask = (register.simd_eq(comma_splat)
            | register.simd_eq(hyphen_splat))
        .to_bitmask();

        while mask != 0 {
            let trailing = mask.trailing_zeros();
            let end = i + trailing as usize;

            let mut num = 0;
            for j in start..end {
                unsafe {
                    num = num * 10 + (*ptr.add(j) & 0x0F) as u64;
                }
            }

            if num1 == 0 {
                num1 = num;
            } else if num2 == 0 {
                num2 = num;

                let (a1, a2) = single_sum(num1 - 1);
                let (b1, b2) = single_sum(num2);

                p1 += b1 - a1;
                p2 += b2 - a2;

                num1 = 0;
                num2 = 0;
            }

            start = end + 1;
            mask ^= 1 << trailing;
        }

        i += 64;
    }

    while i <= bytes.len() {
        let next = unsafe { *ptr.add(i) };
        if i == len || next == b',' || next == b'-' {
            let end = i;

            let mut num = 0;

            for j in start..end {
                unsafe {
                    num = num * 10 + (*ptr.add(j) & 0x0F) as u64;
                }
            }

            if num1 == 0 {
                num1 = num;
            } else if num2 == 0 {
                num2 = num;

                let (a1, a2) = single_sum(num1 - 1);
                let (b1, b2) = single_sum(num2);

                p1 += b1 - a1;
                p2 += b2 - a2;

                num1 = 0;
                num2 = 0;
            }

            start = end + 1;
        }

        i += 1;
    }

    (p1, p2)
}

pub fn part1(input: &Input) -> u64 {
    input.0
}

pub fn part2(input: &Input) -> u64 {
    input.1
}

fn p(r: u8, q: u8) -> u64 {
    (pow10(r * q) - 1) / (pow10(q) - 1)
}

fn t(a: u64, r: u8, q: u8) -> u64 {
    (a / p(r, q)).min(pow10(q) - 1)
}

fn double_sum_between(a: u64, b: u64) -> u64 {
    (a + b) * (b - a + 1)
}

fn single_sum_r(n: u64, r: u8) -> u64 {
    let mut sum = 0;
    let mut q = 1;

    // r * q < 19 only required so the result fits in 64 bits. It is not
    // required for the math itself to work -- log2(10^19) = 63.1166...
    while r * q < 20 && pow10(q - 1) * p(r, q) <= n {
        let a = pow10(q - 1);
        let b = t(n, r, q);
        sum += p(r, q) * double_sum_between(a, b);
        q += 1;
    }

    sum
}

fn single_sum(n: u64) -> (u64, u64) {
    let p1 = single_sum_r(n, 2);
    let mut p2 = p1 as i64;

    for r in 3..num_length(n) + 1 {
        let sign = DOUBLE_COUNT_REMOVAL[r as usize];
        let val = single_sum_r(n, r) as i64;
        p2 += val * sign as i64;
    }

    (p1 >> 1, p2 as u64 >> 1)
}
