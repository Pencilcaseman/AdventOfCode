use aoc::year2024::day06::*;

// #...
// ***#
// ^.*.
// ..*.
const MINIMAL_INPUT: &str = "\
#...
...#
^...
....";

// .#...
// .+-+#
// O^-+.
// ...#.";
const MINIMAL_PART_2: &str = "\
..#.....
......#.
........
..^.....
.....#..";

const CONTAINS_LOOP: &str = "\
..#.....
......#.
........
.#^.....
.....#..";

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
fn test_contains_loop() {
    let input = parse(CONTAINS_LOOP);
    assert!(trace_route(input.0.clone(), (3, 2), (-1, 0)));

    let input = parse(MINIMAL_PART_2);
    assert!(!trace_route(input.0.clone(), (3, 2), (-1, 0)));
}

#[test]
fn test_part2_minimal() {
    let input = parse(MINIMAL_PART_2);
    assert_eq!(part2(&input), 1);
}

#[test]
fn test_part2() {
    let input = parse(EXAMPLE_INPUT);
    assert_eq!(part2(&input), 6);
}
