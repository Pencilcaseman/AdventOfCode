use aoc::year2024::day10::*;

const EXAMPLE_PART_1: &str = "\
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

#[test]
fn test_part1() {
    let example = parse(EXAMPLE_PART_1);
    assert_eq!(part1(&example), 36);
}

#[test]
fn test_part2() {
    let example = parse(EXAMPLE_PART_1);
    assert_eq!(part2(&example), 81);
}
