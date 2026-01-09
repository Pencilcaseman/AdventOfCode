use crate::util::parse::ParseUnsigned;

const PRESENT_SIZE: u32 = 9;
const NUM_PRESENTS: usize = 6;

type Input = u32;

pub fn parse(input: &str) -> Input {
    let mut nums = ParseUnsigned::<u8>::new(input.bytes()).skip(NUM_PRESENTS);

    let mut trivial = 0;

    while let Some(width) = nums.next() {
        let width = (width as u32 / 3) * 3;
        let height = (nums.next().unwrap() as u32 / 3) * 3;
        let area = width * height;

        let max_area =
            nums.by_ref().take(NUM_PRESENTS).map(|x| x as u32).sum::<u32>()
                * PRESENT_SIZE;

        if max_area <= area {
            trivial += 1
        }
    }

    trivial
}

pub fn part1(input: &Input) -> u32 {
    *input
}

pub fn part2(_input: &Input) -> i8 {
    -1
}

// Answers for my input:
// Part 1: 469
// Part 2: No answer required :D
