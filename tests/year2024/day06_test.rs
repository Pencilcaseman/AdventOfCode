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

const EXAMPLE_LOOPBACK: &str = "\
....#....
.....#...
....^....
.........
....#....";

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
........
..^...#.
........
.#......
.....#..";

const CONTAINS_LOOP_2: &str = "\
...........
.#^......#.
........#..";

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

//   0123456789
// 0 ....#.....
// 1 .........#
// 2 ..........
// 3 ..#.......
// 4 .......#..
// 5 ..........
// 6 .#.O^.....
// 7 ........#.
// 8 #.........
// 9 ......#...
//
// (6, 3)
// (7, 6)
// (7, 7)
// (8, 1)
// (8, 3)
// (9, 7)

#[test]
fn test_part1_minimal() {
    let input = parse(MINIMAL_INPUT);
    assert_eq!(part1(&input), 6);
}

#[test]
fn test_part1_loopback() {
    let input = parse(EXAMPLE_LOOPBACK);
    assert_eq!(part1(&input), 7);
}

#[test]
fn test_part1() {
    let input = parse(EXAMPLE_INPUT);
    assert_eq!(part1(&input), 41);
}

#[test]
fn test_contains_loop() {
    let input = parse(CONTAINS_LOOP);

    // let mut grid = input.0;
    let grid = input.0;
    let skipper = Skipper::from(&grid);
    let pos = (input.1 .0 as isize, input.1 .1 as isize);
    let size = input.2;
    let dir = Direction::Up;

    // assert!(is_loop(&mut grid, pos, size, dir));
    assert!(is_loop(&skipper, pos, size, dir));
}

#[test]
fn test_contains_no_loop() {
    let input = parse(MINIMAL_PART_2);

    // let mut grid = input.0;
    let grid = input.0;
    let skipper = Skipper::from(&grid);
    let pos = (input.1 .0 as isize, input.1 .1 as isize);
    let size = input.2;
    let dir = Direction::Up;

    // assert!(!is_loop(&mut grid, pos, size, dir));
    assert!(!is_loop(&skipper, pos, size, dir));
}

#[test]
fn test_contains_loop_2() {
    let input = parse(CONTAINS_LOOP_2);

    // let mut grid = input.0;
    let grid = input.0;
    let skipper = Skipper::from(&grid);
    let pos = (input.1 .0 as isize, input.1 .1 as isize);
    let size = input.2;
    let dir = Direction::Up;

    // assert!(is_loop(&mut grid, pos, size, dir));
    assert!(is_loop(&skipper, pos, size, dir));
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
