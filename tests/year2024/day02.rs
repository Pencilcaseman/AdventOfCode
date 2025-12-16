use aoc::year2024::day02::*;

const EXAMPLE_INPUT: &str = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

const EXAMPLE_PART1_CUSTOM: &str = "\
1 2 3 4 6 7 10 8
1 2 3";

const EXAMPLE_PART2_CUSTOM: &str = "\
5 10 4 3 2 1
9 8 7 6 5 4 3 2 1
9 8 7 6   4 3 2 1
9 8 7 6     3 2 1
9 8 7 6       2 1
9 8 7 6 6 4 3 2 1
9 8 7 6 6   3 2 1
9 8 7 6 6     2 1

1 2 10 3

10

1 1 2

7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

const ANOTHER_EXAMPLE_PART2: &str = "\
1 1 2
2 1 2
1 3 3 4

10 1 2 3
1 2 3 10
5 4 5 6 7

1 2
1 10
10 1

1 2 3 2 3
";

// 18 21 23 26 28 26 27 28 # DOES NOT WORK

#[test]
fn test_sample_part1() {
    let input = parse(EXAMPLE_INPUT);
    assert_eq!(part1(&input), 2);
}

#[test]
fn test_sample_part1_custom() {
    let input = parse(EXAMPLE_PART1_CUSTOM);
    assert_eq!(part1(&input), 1);
}

#[test]
fn test_sample_part2() {
    let input = parse(EXAMPLE_INPUT);
    assert_eq!(part2(&input), 4);
}

#[test]
fn test_sample_part2_custom() {
    let input = parse(EXAMPLE_PART2_CUSTOM);
    assert_eq!(part2(&input), 13);
}

#[test]
fn test_another_sample_part2() {
    let input = parse(ANOTHER_EXAMPLE_PART2);
    assert_eq!(part2(&input), 9);
}
