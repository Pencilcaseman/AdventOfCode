use aoc::year2025::day04::*;

const EXAMPLE_INPUT: &str = "\
987654321111111
811111111111119
234234234234278
818181911112111";

#[test]
fn test_part1() {
    let input = parse(EXAMPLE_INPUT);
    assert_eq!(part1(&input), 0);
}

#[test]
fn test_part2() {
    let input = parse(EXAMPLE_INPUT);
    assert_eq!(part2(&input), 0);
}
