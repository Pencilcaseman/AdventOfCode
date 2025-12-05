use crate::util::integer::pow10;

type Input<'a> = Vec<Vec<u64>>;

pub fn parse<'a>(input: &'a str) -> Input<'a> {
    input
        .lines()
        .map(|x| x.bytes().map(|b| (b - b'0') as u64).collect())
        .collect()
}

pub fn part1(input: &Input) -> u64 {
    solve::<2>(input)
}

pub fn part2(input: &Input) -> u64 {
    solve::<12>(input)
}

pub fn solve<const N: usize>(input: &Input) -> u64 {
    let mut sum = 0;

    for line in input {
        let mut num = [0u64; N];
        let n = line.len();

        let mut current = 0;

        for pos in 0..N {
            let (idx, val) =
                line[current..(n - (N - pos - 1))].iter().enumerate().fold(
                    (0, 0),
                    |(i, v), (ii, vv)| if *vv > v { (ii, *vv) } else { (i, v) },
                );

            num[pos] = val;
            current += idx + 1;
        }

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
