#![warn(clippy::pedantic, clippy::nursery)]

type Input = Vec<usize>;

#[must_use]
pub fn parse(input: &str) -> Input {
    input.bytes().map(|b| (b - b'0') as usize).collect()
}

#[must_use]
pub fn part1(input: &Input) -> usize {
    let mut real_index = 0;
    let mut sim_index = 0;
    let mut end_index = input.len() - 1;
    let mut is_value = true;
    let mut sum = 0;

    let mut string = input.clone();

    loop {
        if is_value {
            sum += sim_index * (real_index / 2);
            string[real_index] -= 1;
        } else {
            sum += sim_index * (end_index / 2);
            string[real_index] -= 1;
            string[end_index] -= 1;
        }

        while string[real_index] == 0 {
            real_index += 1;
            is_value = !is_value;

            if real_index == input.len() {
                break;
            }
        }

        while string[end_index] == 0 {
            // Skip the gap
            end_index -= 2;

            if end_index == 0 {
                break;
            }
        }

        if string[real_index] == 0 || string[end_index] == 0 {
            break;
        }

        sim_index += 1;
    }

    sum
}

#[must_use]
pub fn part2(input: &Input) -> usize {
    0
}

// For my input, the correct answer is:
// Part 1:
// Part 2:
