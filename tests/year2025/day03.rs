use aoc::year2025::day03::*;

const EXAMPLE_INPUT: &str = "\
987654321111111
811111111111119
234234234234278
818181911112111";

// SIMD parser requires lines to be >16 bytes
// #[test]
// fn test_part1_custom() {
//     let input = parse("123211991");
//     assert_eq!(part1(&input), 99);

//     let input = parse("923211881919");
//     assert_eq!(part1(&input), 99);

//     let input = parse("1211");
//     assert_eq!(part1(&input), 21);
// }

// #[test]
// fn test_part1() {
//     let input = parse(EXAMPLE_INPUT);
//     assert_eq!(part1(&input), 357);
// }

#[test]
fn test_part2() {
    let input = parse(EXAMPLE_INPUT);
    assert_eq!(part2(&input), 3121910778619);
}

#[test]
fn test_part2_custom() {
    let input = parse("12321191");
    assert_eq!(solve::<3>(&input), 391);

    let input = parse("123211991");
    assert_eq!(solve::<4>(&input), 3991);

    let input = parse("1232419911");
    assert_eq!(solve::<4>(&input), 9911);
}

//
// 1232419911
//
// 123
// 232
// 324
// 341
// 419
// 499
// 991
// 991
// Done
//
