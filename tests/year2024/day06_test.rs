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
fn test_part1() {
    let input = parse(EXAMPLE_INPUT);
    assert_eq!(part1(&input), 41);
}

// #[test]
// fn test_contains_loop() {
//     let input = parse(CONTAINS_LOOP);
//     assert!(matches!(
//         trace_route(input.0.clone(), (3, 2), (-1, 0)),
//         TraceResult::Loop(_)
//     ));
//
//     let input = parse(MINIMAL_PART_2);
//     assert!(matches!(
//         trace_route(input.0.clone(), (3, 2), (-1, 0)),
//         TraceResult::Exit(_)
//     ));
// }

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
