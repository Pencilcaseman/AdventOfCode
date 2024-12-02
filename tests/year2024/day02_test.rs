use aoc::year2024::day02::*;

const EXAMPLE_PART: &str = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

const EXAMPLE_PART1_CUSTOM: &str = "\
1 2 3 4 6 7 10 7
1 2 3";

#[test]
fn test_sample_part1() {
    let input = parse(EXAMPLE_PART);
    assert_eq!(part1(&input), 2);
}

#[test]
fn test_sample_part1_custom() {
    let input = parse(EXAMPLE_PART1_CUSTOM);
    assert_eq!(part1(&input), 0);
}

#[test]
fn test_sample_part2() {
    let input = parse(EXAMPLE_PART);
    // assert_eq!(part2(&input), 31);
}
