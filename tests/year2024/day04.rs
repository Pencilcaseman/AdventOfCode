use aoc::year2024::day04::*;

const MINIMAL_INPUT_PART1: &str = "\
..X...
.SAMX.
.A..A.
XMAS.S
.X....";

const MINIMAL_INPUT_TARGET_PART1: u32 = 4;

const SMALL_INPUT: &str = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

const SMALL_INPUT_TARGET_PART1: u32 = 18;

const MINIMAL_INPUT_PART2: &str = "\
M.S
.A.
M.S";

const MINIMAL_INPUT_TARGET_PART2: u32 = 1;
const SMALL_INPUT_TARGET_PART2: u32 = 9;

#[test]
fn test_part1_minimal() {
    let input = parse(MINIMAL_INPUT_PART1);
    assert_eq!(part1(&input), MINIMAL_INPUT_TARGET_PART1);
}

#[test]
fn test_part1_small() {
    let input = parse(SMALL_INPUT);
    assert_eq!(part1(&input), SMALL_INPUT_TARGET_PART1);
}

#[test]
fn test_part2_minimal() {
    let input = parse(MINIMAL_INPUT_PART2);
    assert_eq!(part2(&input), MINIMAL_INPUT_TARGET_PART2);
}

#[test]
fn test_part2_small() {
    let input = parse(SMALL_INPUT);
    assert_eq!(part2(&input), SMALL_INPUT_TARGET_PART2);
}
