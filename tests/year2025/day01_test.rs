use aoc::year2025::day01::*;
use num_traits::Euclid;

const EXAMPLE_INPUT: &str = "\
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

// const MY_EXAMPLE: &str = "\
// R50
// R50
// R100
// R150";

const MY_EXAMPLE: &str = "\
L50
L50
L100
L150";

#[test]
fn test_sample_part1() {
    let input = parse(EXAMPLE_INPUT);
    println!("{input:?}");
    assert_eq!(part1(&input), 3);
}

#[test]
fn test_sample_part2() {
    println!("{:?}", (-450).div_rem_euclid(&100));

    let input = parse(EXAMPLE_INPUT);
    assert_eq!(part2(&input), 6);
}

#[test]
fn test_my_example_part2() {
    let input = parse(MY_EXAMPLE);
    assert_eq!(part2(&input), 4);
}
