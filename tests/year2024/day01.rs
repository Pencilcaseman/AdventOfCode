use aoc::year2024::day01::*;

const EXAMPLE_PART: &str = "\
3   4
4   3
2   5
1   3
3   9
3   3";

#[test]
fn test_sample_part1() {
    let input = parse(EXAMPLE_PART);
    assert_eq!(part1(&input), 11);
}

#[test]
fn test_sample_part2() {
    let input = parse(EXAMPLE_PART);
    assert_eq!(part2(&input), 31);
}
