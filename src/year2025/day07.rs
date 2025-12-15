//! # Laboratories
//!
//! ## Part 2
//!
//! Starting with a single beam starting at S, keep track of the total number
//! of ways of reaching any given point by summing merging beams:
//!
//! .......1.......
//! .......1.......
//! ......1^1......
//! ......1.1......
//! .....1^2^1.....
//! .....1.2.1.....
//! ....1^3^3^1....
//! ....1.3.3.1....
//! ...1^4^331^1...
//! ...1.4.331.1...
//! ..1^5^434^2^1..
//! ..1.5.434.2.1..
//! .1^154^74.21^1.
//! .1.154.74.21.1.
//! |^|^|^|^|^|||^| # double-digits required at this point
//! |.|.|.|.|.|||.|
//!
//! ## Part 1
//!
//! Using the same algorithm, part 1 can be solved by counting when a split
//! happens.
//!
//! ## Optimizations
//!
//! - Combine solvers for part 1 and part 2 (very easy in this case)
//! - Odd-indexed rows contain only '.'s and can be ignored
//! - Tachyon beams always start in the middle of the input, so the first row
//!   can be ignored
//! - Splitters appear at even offsets from the first splitter in a row, so odd
//!   indices can be ignored

type Input = (u64, u64);

pub fn parse(input: &str) -> Input {
    let lines: Vec<_> =
        input.lines().step_by(2).skip(1).map(str::as_bytes).collect();

    let len = lines[0].len();
    let mid = len / 2;

    let mut part1 = 0;
    let mut current = vec![0u64; len];

    current[mid] = 1;

    let possible_range = |row| ((mid - row)..(mid + row + 1)).step_by(2);

    for (row, line) in lines.iter().enumerate() {
        for i in possible_range(row) {
            let tmp = current[i];

            if tmp > 0 && line[i] == b'^' {
                part1 += 1;

                current[i] = 0;
                current[i - 1] += tmp;
                current[i + 1] += tmp;
            }
        }
    }

    (part1, current.into_iter().sum())
}

pub fn part1(input: &Input) -> u64 {
    input.0
}

pub fn part2(input: &Input) -> u64 {
    input.1
}

// Answers for my input:
// Part 1: 1662
// Part 2: 40941112789504
