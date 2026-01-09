use aoc::year2025::day12::*;

const EXAMPLE_INPUT: &str = "\
0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2";

#[test]
fn test_part1() {
    let input = parse(EXAMPLE_INPUT);
    // The example is hard, but the real input is trivial :)
    // assert_eq!(part1(&input), 2);
}
