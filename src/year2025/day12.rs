// All presents are 3x3
// 6 present shapes?

use crate::util::parse::ParseUnsigned;

type Present = u8;

#[derive(Debug)]
pub struct Tree {
    rows: u8,
    cols: u8,
    counts: Vec<u8>,
}

type Input = (Vec<Present>, Vec<Tree>);

pub fn parse(input: &str) -> Input {
    let mut presents = Vec::new();
    let mut trees = Vec::new();

    // TODO: More efficient parsing algorithm?

    let mut parsing_presents = true;
    let mut current = 0;

    for line in input.lines() {
        // Check if this line is not a present
        if line.contains('x') {
            parsing_presents = false;
        }

        if parsing_presents {
            if line.is_empty() {
                presents.push(current);
                current = 0;
            } else {
                current += line.bytes().filter(|&b| b == b'#').count() as u8;
            }
        } else {
            let mut nums = ParseUnsigned::<u8>::new(line.bytes());

            let rows = nums.next().unwrap();
            let cols = nums.next().unwrap();
            let counts = nums.collect();

            trees.push(Tree { rows, cols, counts });
        }
    }

    (presents, trees)
}

pub fn part1((presents, trees): &Input) -> u32 {
    let mut impossible = 0;
    let mut trivial = 0;

    for tree in trees {
        // Presents cannot possibly fit under the tree
        let min_area: u32 = tree
            .counts
            .iter()
            .zip(presents)
            .map(|(&count, &area)| (count as u32) * (area as u32))
            .sum();

        let tree_area = tree.rows as u32 * tree.cols as u32;
        if min_area > tree_area {
            impossible += 1;
        }

        // Presents fit trivially under the tree
        let rounded_rows = (tree.rows as u32 / 3) * 3;
        let rounded_cols = (tree.cols as u32 / 3) * 3;
        let rounded_area = rounded_rows * rounded_cols;

        // All presents are 3x3
        let max_area =
            9 * tree.counts.iter().copied().map(u32::from).sum::<u32>();

        if max_area <= rounded_area {
            trivial += 1;
        }
    }

    println!("Total: {}", trees.len());
    println!("Impossible: {impossible}");
    println!("Trivial: {trivial}");

    trivial
}

pub fn part2(input: &Input) -> u32 {
    0
}

// Answers for my input:
// Part 1:
// Part 2:
