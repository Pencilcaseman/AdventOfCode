use aoc::year2024::day14::*;

const EXAMPLE_SINGLE: &str = "p=2,4 v=2,-3";

const EXAMPLE_1: &str = "\
p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";

#[test]
fn test_part1() {
    let parsed = parse(EXAMPLE_SINGLE);
    assert_eq!(solve_part1(&parsed, 11, 7, 5), 0);

    let parsed = parse(EXAMPLE_1);
    assert_eq!(solve_part1(&parsed, 11, 7, 100), 12);
}

#[test]
fn test_part2() {}
