use aoc::year2025::day10::*;

const EXAMPLE_INPUT: &str = "\
[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";

// const SLOWEST_EXAMPLE: &str = "\
// [.##.#.###] (0,2,3,4) (3,4,6) (0,3,8) (0,1,5,6,7,8) (2,3) (0,1,2,4)
// (0,1,3,5,6,8) (0,2,4,5,6,8) (0,4,5,8) (1,3,4,5,8)
// {39,3,17,32,48,27,33,0,38}";

const SLOWEST_EXAMPLE: &str = "\
[#..##.###.] (6,7,8) (3,5,7) (2,4) (1,3,4,9) (0,1,2,3,6,7,9) (0,1,2,3,5,8) (3,8) (2,3,4,6,7,8,9) (3,4,7,8) (0,1,2,3,4,5,7,8) (0,1,2,4,7) (2,4,6) (5,6,8,9) {41,59,70,115,88,248,237,94,286,230}";

const NOT_WORKING: &str = "\
[###..#] (0,1,2,4,5) (0,4,5) (2,3) (1,3) (2,3,4,5) (0,1,4,5) (2,5) {33,38,44,37,46,51}";

#[test]
fn test_part1() {
    // let input = parse(EXAMPLE_INPUT);
    // assert_eq!(part1(&input), 7);
}

#[test]
fn test_part2() {
    let input = parse(EXAMPLE_INPUT);
    assert_eq!(part2(&input), 33);
}

#[test]
fn test_slow_example() {
    let input = parse(SLOWEST_EXAMPLE);
    assert_eq!(part2(&input), 0);
}

#[test]
fn test_not_working() {
    let input = parse(NOT_WORKING);
    assert_eq!(part2(&input), 0);
}
