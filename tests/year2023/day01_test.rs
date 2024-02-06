#[cfg(test)]
mod tests {
    use aoc::year2023::day01::*;

    const EXAMPLE_PART1: &str = "\
        1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet";

    const EXAMPLE_PART2: &str = "\
        two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen";

    #[test]
    fn test_sample_part1() {
        let input = parse(EXAMPLE_PART1);
        assert_eq!(part1(&input), 142);
    }

    #[test]
    fn test_sample_part2() {
        let input = parse(EXAMPLE_PART2);
        assert_eq!(part2(&input), 281);
    }
}
