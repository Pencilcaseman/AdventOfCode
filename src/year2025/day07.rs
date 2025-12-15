type Input = (u64, u64);

pub fn parse(input: &str) -> Input {
    let mut part1 = 0;
    let mut current = [0u64; 192];

    for (_, line) in input.lines().enumerate().filter(|(i, _)| i % 2 == 0) {
        line.bytes().enumerate().for_each(|(i, b)| match b {
            b'S' => {
                current[i] = 1;
            }
            b'^' => {
                if current[i] > 0 {
                    part1 += 1;

                    current[i - 1] += current[i];
                    current[i + 1] += current[i];
                    current[i] = 0;
                }
            }
            _ => (),
        });
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
