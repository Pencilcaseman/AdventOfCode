use aoc::year2024::day09::*;

const MINIMAL_EXAMPLE: &str = "12345";
const EXAMPLE_INPUT: &str = "2333133121414131402";

#[test]
fn test_part1_minimal() {
    let input = parse(MINIMAL_EXAMPLE);
    assert_eq!(part1(&input), 60);
}

#[test]
fn test_part1_example() {
    let input = parse(EXAMPLE_INPUT);
    assert_eq!(part1(&input), 1928);
}

#[test]
fn test_part2() {}
