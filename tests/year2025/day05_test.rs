use aoc::year2025::day05::*;

const EXAMPLE_INPUT: &str = "\
3-5
10-14
16-20
12-18

1
5
8
11
17
32";

#[test]
fn test_part1() {
    let input = parse(EXAMPLE_INPUT);
    assert_eq!(part1(&input), 3);
}

#[test]
fn test_part2() {
    let input = parse(EXAMPLE_INPUT);
    assert_eq!(part2(&input), 14);
}
