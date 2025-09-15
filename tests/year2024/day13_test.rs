use aoc::year2024::day13::*;

const EXAMPLE_1: &str = "\
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";

const EDGE_CASE_1: &str = "\
Button A: X+1, Y+2
Button B: X+10, Y+20
Prize: X=105, Y=210";

const EDGE_CASE_2: &str = "\
Button A: X+10, Y+20
Button B: X+1, Y+2
Prize: X=105, Y=210";

const EDGE_CASE_3: &str = "\
Button A: X+100, Y+200
Button B: X+1, Y+2
Prize: X=10, Y=20";

const EDGE_CASE_4: &str = "\
Button A: X+100, Y+200
Button B: X+1, Y+2
Prize: X=11, Y=20";

#[test]
fn test_part1() {
    let parsed = parse(EXAMPLE_1);
    assert_eq!(part1(&parsed), 480);

    let parsed = parse(EDGE_CASE_1);
    assert_eq!(part1(&parsed), 25);

    let parsed = parse(EDGE_CASE_2);
    assert_eq!(part1(&parsed), 35);

    let parsed = parse(EDGE_CASE_3);
    assert_eq!(part1(&parsed), 10);

    let parsed = parse(EDGE_CASE_4);
    assert_eq!(part1(&parsed), 0);
}

#[test]
fn test_part2() {}
