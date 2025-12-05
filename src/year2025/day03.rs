type Input<'a> = Vec<&'a str>;

pub fn parse<'a>(input: &'a str) -> Input<'a> {
    input.lines().collect()
}

pub fn part1(input: &Input) -> u64 {
    solve::<2>(input)
}

pub fn part2(input: &Input) -> u64 {
    solve::<12>(input)
}

pub fn solve<const N: usize>(input: &Input) -> u64 {
    let mut num = [0u8; N];

    input
        .iter()
        .map(|line| {
            let end = line.len() - N;
            num.copy_from_slice(&line.as_bytes()[end..]);

            for mut tmp in line[..end].bytes().rev() {
                for j in &mut num {
                    if tmp < *j {
                        break;
                    }
                    tmp = std::mem::replace(j, tmp);
                }
            }

            num.into_iter().fold(0, |t, n| t * 10 + (n - b'0') as u64)
        })
        .sum()
}

// Answers for my input:
// Part 1: 17263
// Part 2: 170731717900423
