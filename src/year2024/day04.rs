#![warn(clippy::pedantic, clippy::nursery)]

#[must_use]
pub fn parse(input: &str) -> Vec<Vec<u8>> {
    input.lines().map(|line| line.as_bytes().to_vec()).collect()
}

#[allow(clippy::cast_sign_loss, clippy::cast_possible_wrap)]
#[must_use]
pub fn part1(input: &[Vec<u8>]) -> u32 {
    // From a given point, we can go:
    //  - Up
    //  - Down
    //  - Left
    //  - Right
    //  - Up-Left
    //  - Up-Right
    //  - Down-Left
    //  - Down-Right
    //
    // If we only allow the current point to be the starting value, we will only
    // find unique occurrences of 'XMAS'

    const XMAS: [u8; 4] = *b"XMAS";

    let num_rows = input.len();
    let num_cols = input[0].len();
    let mut count = 0;

    for row in 0..num_rows {
        for col in 0..num_cols {
            if input[row][col] != b'X' {
                continue;
            }

            for direction in [
                (0isize, 1isize), // Right
                (0, -1),          // Left
                (1, 0),           // Down
                (-1, 0),          // Up
                (1, 1),           // Down-Right
                (-1, 1),          // Down-Left
                (1, -1),          // Up-Right
                (-1, -1),         // Up-Left
            ] {
                let mut idx = 0;
                let mut tmp_row = row;
                let mut tmp_col = col;

                while idx < XMAS.len()
                    && tmp_row < num_rows
                    && tmp_col < num_cols
                    && input[tmp_row][tmp_col] == XMAS[idx]
                {
                    idx += 1;

                    tmp_row = (tmp_row as isize + direction.0) as usize;
                    tmp_col = (tmp_col as isize + direction.1) as usize;
                }

                if idx == XMAS.len() {
                    count += 1;
                }
            }
        }
    }

    count
}

#[must_use]
pub fn part2(input: &[Vec<u8>]) -> u32 {
    // From any given point, we use it as the centre 'A' of 'MAS'

    // ord('S') - ord('M') = 6
    const DIFF: u8 = 6;

    let mut count = 0;

    for row in 1..input.len() - 1 {
        for col in 1..input[0].len() - 1 {
            if input[row][col] != b'A' {
                continue;
            }

            // M S    S S
            //  A      A
            // M S    M M

            let vertical = input[row - 1][col - 1] == input[row + 1][col - 1]
                && input[row - 1][col + 1] == input[row + 1][col + 1]
                && input[row - 1][col - 1].abs_diff(input[row - 1][col + 1])
                    == DIFF;

            let horizontal = input[row - 1][col - 1] == input[row - 1][col + 1]
                && input[row + 1][col - 1] == input[row + 1][col + 1]
                && input[row - 1][col - 1].abs_diff(input[row + 1][col - 1])
                    == DIFF;

            if vertical || horizontal {
                count += 1;
            }
        }
    }

    count
}

// For my input, the correct answer is:
// Part 1: 2603
// Part 2: 1965
