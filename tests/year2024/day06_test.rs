use aoc::year2024::day06::*;

const MINIMAL_INPUT: &str = "\
#...
...#
^...
....";

// #...
// ***#
// ^.*.
// ..*.

const EXAMPLE_INPUT: &str = "\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

#[test]
fn test_part1_minimal() {
    let input = parse(MINIMAL_INPUT);
    assert_eq!(part1(&input), 6);
}

#[test]
fn test_part1() {
    let input = parse(EXAMPLE_INPUT);
    assert_eq!(part1(&input), 41);
}

#[test]
fn test_part2() {}
