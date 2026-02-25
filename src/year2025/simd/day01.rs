type Input = (i32, i32);

use std::{
    hint::unreachable_unchecked,
    simd::{Simd, cmp::SimdPartialEq, u8x64},
};

pub fn parse(input: &str) -> Input {
    let bytes = input.as_bytes();
    let ptr = bytes.as_ptr();
    let len = bytes.len();
    let mut i = 0;

    let mut p1 = 0;
    let mut p2 = 0;
    let mut angle = 50;

    let newline_mask = Simd::splat(b'\n');
    let mut prev_end = 0;

    while i + 64 <= len {
        let chunk = u8x64::from_slice(&bytes[i..]);
        let eq = chunk.simd_eq(newline_mask);
        let mut mask = eq.to_bitmask();

        while mask != 0 {
            let trailing = mask.trailing_zeros() as usize;
            let end = i + trailing;
            let start = prev_end;

            let len = end - start;
            debug_assert!(len < 5, "Invalid input");

            unsafe {
                line_solve(ptr, start, len, &mut angle, &mut p1, &mut p2);
            }

            prev_end = end + 1;
            mask ^= 1 << trailing;
        }
        i += 64;
    }

    // Handle remaining elements
    while i <= len {
        if i == len || bytes[i] == b'\n' {
            let start = prev_end;
            let end = i;

            let len = end - start;
            debug_assert!(len < 5, "Invalid input");

            unsafe {
                line_solve(ptr, start, len, &mut angle, &mut p1, &mut p2);
            }

            prev_end = end + 1;
        }
        i += 1;
    }

    (p1, p2)
}

#[inline(always)]
unsafe fn line_solve(
    ptr: *const u8,
    start: usize,
    len: usize,
    angle: &mut i32,
    p1: &mut i32,
    p2: &mut i32,
) {
    debug_assert!(
        2 <= len && len <= 4,
        "Length must be between 2 and 4, inclusive."
    );

    unsafe {
        // More efficient than looping.
        // Safe because (in debug mode) we check len is valid
        let mag = match len {
            2 => (*ptr.add(start + 1) & 0x0F) as i32,
            3 => {
                (*ptr.add(start + 1) & 0x0F) as i32 * 10
                    + (*ptr.add(start + 2) & 0x0F) as i32
            }
            4 => {
                (*ptr.add(start + 1) & 0x0F) as i32 * 100
                    + (*ptr.add(start + 2) & 0x0F) as i32 * 10
                    + (*ptr.add(start + 3) & 0x0F) as i32
            }
            _ => unreachable_unchecked(),
        };

        if *ptr.add(start) == b'R' {
            *p2 += (*angle + mag) / 100;
            *angle = (*angle + mag) % 100;
        } else {
            // Treat left as reversed right.
            let rev_angle = if *angle == 0 { 0 } else { 100 - *angle };
            *p2 += (rev_angle + mag) / 100;

            *angle -= mag;
            if *angle < 0 {
                *angle = (*angle % 100 + 100) % 100;
            }
        }

        *p1 += (*angle == 0) as i32;
    }
}

pub fn part1(input: &Input) -> i32 {
    input.0
}

pub fn part2(input: &Input) -> i32 {
    input.1
}
