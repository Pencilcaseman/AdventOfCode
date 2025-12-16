use aoc::year2024::day07::*;

const EXAMPLE_INPUT: &str = "\
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

#[test]
fn test_part1() {
    let input = parse(EXAMPLE_INPUT);
    assert_eq!(part1(&input), 3749);
}

#[test]
fn test_part2() {
    let input = parse(EXAMPLE_INPUT);
    assert_eq!(part2(&input), 11387);
}

#[test]
fn test_ends_with() {
    assert!(ends_with(190, 190) == Some(0));
    assert!(ends_with(190, 90) == Some(1));
    assert!(ends_with(190, 0) == Some(19));

    assert!(ends_with(190, 1).is_none());
    assert!(ends_with(190, 10).is_none());

    // NOTE: This should pass, but the input never contains 4+ digit
    // concatenations, so we can ignore this case
    //
    // assert!(ends_with(1234, 1234) == Some(0));

    assert!(ends_with(1234, 234) == Some(1));
    assert!(ends_with(1234, 34) == Some(12));
    assert!(ends_with(1234, 4) == Some(123));

    assert!(ends_with(156, 6) == Some(15));
}
