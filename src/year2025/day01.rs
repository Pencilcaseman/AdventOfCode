//! # Secret Entrance
//!
//! Part 1 is quite simple -- just keep track of the angle of the dial
//! (modulo 100) and count each time it lands on zero.
//!
//! Part 2 is a little more tricky, mainly due to left rotations.
//!
//! To improve performance slightly, we can "stream" the input bytes and
//! "generate" each value as bytes come in, processing them when a new-line is
//! seen (hence the need for the extra `std::iter::once(b'\n')`.)

use num_traits::Euclid;

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
                let (full, rem) = num.div_rem_euclid(&100);

                p2 += full;

                if dir == b'R' {
                    angle += rem;

                    if angle > 99 {
                        angle -= 100;
                        p2 += 1;
                    }
                } else if dir == b'L' {
                    let mut new_angle = angle - rem;

                    if new_angle < 0 {
                        new_angle += 100;

                        // Passed zero
                        if angle > 0 {
                            p2 += 1;
                        }
                    } else if new_angle == 0 {
                        // Landed on zero
                        p2 += 1;
                    }

                    angle = new_angle;
                }

                if angle == 0 {
                    p1 += 1;
                }

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
