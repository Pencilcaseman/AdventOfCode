use aoc::year2024::day12::*;

const EXAMPLE_1: &str = "\
AAAA
BBCD
BBCC
EEEC";

const EXAMPLE_2: &str = "\
OOOOO
OXOXO
OOOOO
OXOXO
OOOOO";

#[test]
fn test_part1() {
    let parsed = parse(EXAMPLE_1);
    assert_eq!(part1(&parsed), 140);

    let parsed = parse(EXAMPLE_2);
    assert_eq!(part1(&parsed), 772);
}

#[test]
fn test_part2() {}
