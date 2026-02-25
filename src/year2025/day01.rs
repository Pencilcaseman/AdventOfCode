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

pub fn part1(input: &Input) -> i32 {
    input.0
}

pub fn part2(input: &Input) -> i32 {
    input.1
}

// Answers for my input
// Part 1: 1078
// Part 2: 6412
