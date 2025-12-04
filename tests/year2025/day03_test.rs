use aoc::year2025::day03::*;

const EXAMPLE_INPUT: &str = "";

#[test]
fn test_part1() {
    let input = parse(EXAMPLE_INPUT);
    assert_eq!(part1(&input), 1227775554);
}

#[test]
fn test_part2() {
    let input = parse(EXAMPLE_INPUT);
    assert_eq!(part2(&input), 4174379265);
}
