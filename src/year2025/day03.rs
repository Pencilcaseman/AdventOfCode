use crate::util::integer::pow10;

type Input<'a> = Vec<Vec<u64>>;

pub fn parse<'a>(input: &'a str) -> Input<'a> {
    input
        .lines()
        .map(|x| x.bytes().map(|b| (b - b'0') as u64).collect())
        .collect()
}

pub fn part1(input: &Input) -> u64 {
    let mut sum = 0;

    for line in input {
        let mut first = line[0];
        let mut second = line[1];

        let mut i = 2;
        let n = line.len();

        while i < n {
            let digit = line[i];

            if digit >= second {
                if second > first {
                    first = second;
                }

                second = digit;
            }

            if second > first && i < n - 1 {
                first = second;
                second = line[i + 1];
                i += 1;
            }

            i += 1;
        }

        sum += first * 10 + second;
    }

    sum
}

pub fn part2(input: &Input) -> u64 {
    solve::<12>(input)
}

pub fn solve<const N: usize>(input: &Input) -> u64 {
    let mut sum = 0;

    for line in input {
        let mut num = [0u64; N];
        let n = line.len();

        num.copy_from_slice(&line[0..N]);

        let mut idx = N;

        while idx < n {
            let mut current = 0;

            while current + 1 < N && idx < n {
                if num[current] < num[current + 1] {
                    num[current..].rotate_left(1);
                    num[N - 1] = line[idx];
                    current = 0;
                    idx += 1;
                } else {
                    current += 1;
                }
            }

            if idx < n && num[N - 1] < line[idx] {
                num[N - 1] = line[idx];
            }

            idx += 1;
        }

        // sum += first * 10 + second;
        sum += num
            .into_iter()
            .enumerate()
            .map(|(exp, num)| num * pow10((N - exp - 1) as u8))
            .sum::<u64>();
    }

    sum
}

// Answers for my input:
// Part 1: 17263
// Part 2: 170731717900423
