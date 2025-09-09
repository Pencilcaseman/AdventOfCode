use aoc::year2024::day11::*;

const EXAMPLE: &str = "125 17";

#[test]
fn test_part1() {
    let parsed = parse(EXAMPLE);
    // let parsed = parse("0 0 0 0 0 1 10");
    assert_eq!(part1(&parsed), 55312);
}

#[test]
fn test_part2() {}
