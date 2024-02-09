use aoc::year2023::day03::*;

const EXAMPLE: &str = "\
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

#[test]
fn test_sample_part1() {
    let input = parse(EXAMPLE);
    assert_eq!(part1(&input), 4361);
}

#[test]
fn test_sample_part2() {
    let input = parse(EXAMPLE);
    assert_eq!(part2(&input), 467_835);
}
