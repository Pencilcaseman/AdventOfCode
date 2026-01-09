use crate::util::parse::ParseUnsigned;

const PRESENT_SIZE: u32 = 9;
const NUM_PRESENTS: usize = 8;

#[derive(Debug)]
pub struct Tree {
    rows: u32,
    cols: u32,
    counts: [u32; NUM_PRESENTS],
}

type Input = Vec<Tree>;

pub fn parse(input: &str) -> Input {
    let mut trees = Vec::new();

    for line in input.lines() {
        let bytes = line.as_bytes();

        if bytes.len() < 3 || bytes[2] != b'x' {
            continue;
        }

        let mut nums = ParseUnsigned::<u8>::new(line.bytes()).map(|x| x as u32);

        let rows = nums.next().unwrap();
        let cols = nums.next().unwrap();

        let mut counts = [0; NUM_PRESENTS];
        nums.enumerate().for_each(|(i, n)| counts[i] = n);

        trees.push(Tree { rows, cols, counts });
    }

    trees
}

pub fn part1(input: &Input) -> u32 {
    let mut trivial = 0;

    for tree in input {
        // Presents fit trivially under the tree
        let rounded_rows = (tree.rows / 3) * 3;
        let rounded_cols = (tree.cols / 3) * 3;
        let rounded_area = rounded_rows * rounded_cols;

        // All presents are 3x3
        let max_area = PRESENT_SIZE * tree.counts.iter().sum::<u32>();

        if max_area <= rounded_area {
            trivial += 1;
        }
    }

    trivial
}

pub fn part2(input: &Input) -> u32 {
    0
}

// Answers for my input:
// Part 1: 469
// Part 2: No answer required :D
