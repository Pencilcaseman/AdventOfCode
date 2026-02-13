//! # Secret Entrance
//!
//! Part 1 is quite simple -- just keep track of the angle of the dial
//! (modulo 100) and count each time it lands on zero.
//!
//! Part 2 is a little more tricky, mainly due to left rotations. To simplify
//! things, we can treat left rotations as reversed right rotations.
//!
//! To improve performance, we parse the input stream using SIMD loads of 16
//! bytes. To do this, we:
//! 1. load 16 bytes from the input
//! 2. Compare the SIMD register against ['\n', '\n', ..., '\n']
//! 3. Convert that into a binary SIMD mask
//!    - This makes the LSB the mask of the first entry in the original register
//! 4. Use .trailing_zeros() to find the indices of the newlines
//! 5. Match-based parsing of substrings because of invariants in the input
//! 6. Solve as above
//! 7. Toggle the bit in the mask and repeat from 1. if possible
//! 8. Scalar logic to parse the remaining input bytes (at most 16 of them)

type Input = (i32, i32);

#[cfg(not(feature = "simd"))]
mod scalar_solution {
    use super::*;

    pub fn parse(input: &str) -> Input {
        let mut p1: i32 = 0;
        let mut p2: i32 = 0;

        let mut angle = 50;

        let mut dir = b'0';
        let mut num = 0;

        for byte in input.bytes().chain(std::iter::once(b'\n')) {
            match byte {
                b'L' => dir = b'L',
                b'R' => dir = b'R',
                b'\n' => {
                    if dir == b'R' {
                        p2 += (angle + num) / 100;
                        angle = (angle + num) % 100;
                    } else {
                        // Treat left rotation as reversed right rotation
                        let reversed = (100 - angle) % 100;
                        p2 += (reversed + num) / 100;
                        angle = (angle - num).rem_euclid(100);
                    }

                    p1 += (angle == 0) as i32;

                    dir = b'0';
                    num = 0;
                }
                digit => num = num * 10 + (digit - b'0') as i32,
            }
        }

        (p1, p2)
    }
}

#[cfg(feature = "simd")]
mod simd_solution {
    use std::{
        hint::unreachable_unchecked,
        simd::{Simd, cmp::SimdPartialEq, u8x64},
    };

    use super::*;

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
}

#[cfg(not(feature = "simd"))]
pub use scalar_solution::parse;
#[cfg(feature = "simd")]
pub use simd_solution::parse;

pub fn part1(input: &Input) -> i32 {
    input.0
}

pub fn part2(input: &Input) -> i32 {
    input.1
}

// Answers for my input
// Part 1: 1078
// Part 2: 6412
