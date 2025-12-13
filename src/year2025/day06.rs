use crate::util::parse::ParseUnsigned;

pub enum Op {
    Sum,
    Prod,
}

type Input<'a> = (Vec<&'a str>, Vec<Op>);

pub fn parse(input: &'_ str) -> Input<'_> {
    let num_lines = input.bytes().filter(|&b| b == b'\n').count() + 1;

    let mut lines = input.splitn(num_lines, '\n');

    let nums: Vec<&str> = lines.by_ref().take(num_lines - 1).collect();
    let ops = lines
        .next()
        .unwrap()
        .bytes()
        .filter_map(|b| match b {
            b'+' => Some(Op::Sum),
            b'*' => Some(Op::Prod),
            _ => None,
        })
        .collect();

    (nums, ops)
}

pub fn part1((nums, ops): &Input) -> u64 {
    let len = ops.len();
    let mut results = vec![0; len];

    for row in nums {
        results
            .iter_mut()
            .zip(ParseUnsigned::<u64>::new(row.bytes()))
            .zip(ops.iter())
            .for_each(|((result, num), op)| match op {
                Op::Sum => *result += num,
                Op::Prod => {
                    if *result == 0 {
                        *result = num;
                    } else {
                        *result *= num;
                    }
                }
            })
    }

    results.into_iter().sum()
}

pub fn part2((nums, ops): &Input) -> u64 {
    let len = nums[0].len();
    let mut vertical_nums = vec![0; len];

    for line in nums {
        for (vert, byte) in vertical_nums.iter_mut().zip(line.bytes()) {
            if byte.wrapping_sub(b'0') < 10 {
                *vert = *vert * 10 + (byte - b'0') as u64;
            }
        }
    }

    ops.iter()
        .zip(vertical_nums.split(|&n| n == 0))
        .map(|(op, vert)| match op {
            Op::Sum => vert.iter().sum::<u64>(),
            Op::Prod => vert.iter().product(),
        })
        .sum()
}

// Answers for my input:
// Part 1: 6209956042374
// Part 2: 12608160008022
