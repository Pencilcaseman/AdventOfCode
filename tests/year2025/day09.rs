use aoc::year2025::day09::*;

const EXAMPLE_INPUT: &str = "\
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";

#[test]
fn test_part1() {
    let input = parse(EXAMPLE_INPUT);
    assert_eq!(part1(&input), 50);
}

#[test]
fn test_part2() {
    let input = parse(EXAMPLE_INPUT);
    assert_eq!(part2(&input), 24);
}
