use aoc::year2024::day09::*;

const MINIMAL_EXAMPLE: &str = "12345";
const EXAMPLE_INPUT: &str = "2333133121414131402";

const MINMAL_EXAMPLE_PART2: &str = "2312304";

#[test]
fn test_part1_minimal() {
    let input = parse(MINIMAL_EXAMPLE);
    assert_eq!(part1(&input), 60);
}

#[test]
fn test_part1_example() {
    let input = parse(EXAMPLE_INPUT);
    assert_eq!(part1(&input), 1928);
}

#[test]
fn test_part2_minimal() {
    let input = parse(MINMAL_EXAMPLE_PART2);
    assert_eq!(part2(&input), 173);
}

#[test]
fn test_part2_custom1() {
    let input = parse("05392");
    assert_eq!(part2(&input), 11);
}

#[test]
fn test_part2_custom2() {
    let input = parse("00000000000000000000000000000000000");
    assert_eq!(part2(&input), 0);
}

#[test]
fn test_part2_custom3() {
    let input = parse("055");
    assert_eq!(part2(&input), 10);
}

#[test]
fn test_part2_custom4() {
    let input = parse("0530101");
    assert_eq!(part2(&input), 11);
}

#[rustfmt::skip]
#[test]
fn test_part2() {
    // 0 0 9 9 2 1 1 1 7 7 7 . 4 4 . 3 3 3 . . . . 5 5 5 5 . 6 6 6 6 . . . . . 8 8 8 8 . .

    // 0 0 . . . 1 1 1 . . . 2 . . . 3 3 3 . 4 4 . 5 5 5 5 . 6 6 6 6 . 7 7 7 . 8 8 8 8 9 9
    // 0 1 2 3 4 5 6 7 8 9 1 1 1 1 1 1 1 1 1 1 2 2 2 2 2 2 2 2 2 2 3 3 3 3 3 3 3 3 3 3 4 4
    //                     0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1

    let input = parse(EXAMPLE_INPUT);
    assert_eq!(part2(&input), 2858);
}
