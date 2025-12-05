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
    let mut sum = 0;

    for line in input {
        // let line = line.as_bytes();
        let mut num = [0u8; N];
        let n = line.len();

        let end = n - N;

        // let mut current = 0;
        //
        // for pos in 0..N {
        //     for i in current..(n - (N - pos - 1)) {
        //         if line[i] > num[pos] {
        //             num[pos] = line[i];
        //             current = i + 1;
        //         }
        //     }
        // }

        num.copy_from_slice(&line.as_bytes()[end..]);

        for mut tmp in line[..end].bytes().rev() {
            for j in 0..N {
                if tmp >= num[j] {
                    let old = num[j];
                    num[j] = tmp;
                    tmp = old;
                } else {
                    // Cannot create a better result
                    break;
                }
            }
        }

        sum += num.into_iter().fold(0, |t, n| t * 10 + (n - b'0') as u64);
    }

    sum
}

// Answers for my input:
// Part 1: 17263
// Part 2: 170731717900423
