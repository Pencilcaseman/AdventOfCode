use aoc::year2025::day06::*;

const EXAMPLE_INPUT: &str = "\
123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";

const SIMD_INPUT_1: &str = "\
123 45    91 1234
2 3  2    82 12  
3     1 2 73 1   
+   +   + +  +   ";

const SIMD_INPUT_2: &str = "\
123 45    91 1234
2 3  2    82 12  
3     1 2 73 1   
*   *   * *  *   ";

const LONGER_INPUT: &str = "\
52 812 541 56 25 2    1 59  428 262  15 65   9 8  361   77 923 69 38 92 49  245
48 554 421 92 25 218  3 28  284 551  99 35  19 83 651  741 756 22 71 78 2  6349
92 731 86  68 93 553 92 26  183   5  84 73 891 23 931 1229 916 32 87 99 9  3115
39 972 8   52 3  435 86 772 56    1 185 78 797 13 549 9795  42 52  7 2  4  6327
+  +   +   *  +  +   *  *   *   *   *   *  +   *  +   +    +   *  *  *  +  +   ";

#[test]
fn test_part1() {
    let input = parse(EXAMPLE_INPUT);
    assert_eq!(part1(&input), 4277556);
}

#[test]
fn test_part2() {
    let input = parse(EXAMPLE_INPUT);
    assert_eq!(part2(&input), 3263827);
}

#[test]
fn test_part2_simd_1() {
    let input = parse(SIMD_INPUT_1);
    let target = 123 + 2 + 33 + 4 + 52 + 1 + 2 + 987 + 123 + 111 + 22 + 3 + 4;
    assert_eq!(part2(&input), target);
}

#[test]
fn test_part2_simd_2() {
    let input = parse(SIMD_INPUT_2);
    let target = 123 * 2 * 33 + 4 * 52 * 1 + 2 + 987 * 123 + 111 * 22 * 3 * 4;
    assert_eq!(part2(&input), target);
}

// #[test]
// fn test_part1_longer_input() {
//     let input = parse(LONGER_INPUT);
//     assert_eq!(part1(&input), 0);
// }
