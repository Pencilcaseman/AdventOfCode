use crate::util::parse::ParseUnsigned;

type Input<'a> = (Vec<&'a str>, &'a str);

pub fn parse(input: &'_ str) -> Input<'_> {
    let num_lines = input.bytes().filter(|&b| b == b'\n').count() + 1;

    let mut lines = input.splitn(num_lines, '\n');

    let nums: Vec<&str> = lines.by_ref().take(num_lines - 1).collect();
    let ops = lines.next().unwrap();

    (nums, ops)
}

pub fn part1((nums, ops): &Input) -> u64 {
    let mut num_iterators: Vec<_> =
        nums.iter().map(|n| ParseUnsigned::<u64>::new(n.bytes())).collect();

    ops.bytes()
        .filter(|&b| b == b'+' || b == b'*')
        .map(|op| {
            let thing =
                num_iterators.iter_mut().map(|iter| iter.next().unwrap());

            if op == b'+' { thing.sum::<u64>() } else { thing.product() }
        })
        .sum()
}

pub fn part2((nums, ops): &Input) -> u64 {
    let len = nums[0].len();
    let mut vertical_nums = vec![0; len];

    for line in nums {
        for (vert, byte) in vertical_nums.iter_mut().zip(line.bytes()) {
            if byte.is_ascii_digit() {
                *vert = *vert * 10 + (byte - b'0') as u64;
            }
        }
    }

    ops.bytes()
        .filter(|&b| b == b'+' || b == b'*')
        .zip(vertical_nums.split(|&n| n == 0))
        .map(|(op, vert)| {
            if op == b'+' {
                vert.iter().sum::<u64>()
            } else {
                vert.iter().product()
            }
        })
        .sum()
}

// Answers for my input:
// Part 1: 6209956042374
// Part 2: 12608160008022
