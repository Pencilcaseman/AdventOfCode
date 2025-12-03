use aoc::year2025::day01::*;

const EXAMPLE_INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

#[test]
fn test_sample_part1() {
    let input = parse(EXAMPLE_INPUT);
    assert_eq!(part1(&input), 1227775554);
}

#[test]
fn test_sample_part2() {}
