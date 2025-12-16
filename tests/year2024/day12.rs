use aoc::year2024::day12::*;

const EXAMPLE_CONTRIVED: &str = "\
AAAAAAAA
ABBBBBBB
ABBBBBBB
ABBBBBBB";

const EXAMPLE_1: &str = "\
AAAA
BBCD
BBCC
EEEC";

const EXAMPLE_2: &str = "\
OOOOO
OXOXO
OOOOO
OXOXO
OOOOO";

const EXAMPLE_3: &str = "\
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

#[test]
fn test_part1() {
    let parsed = parse(EXAMPLE_1);
    assert_eq!(part1(&parsed), 140);

    let parsed = parse(EXAMPLE_2);
    assert_eq!(part1(&parsed), 772);

    let parsed = parse(EXAMPLE_3);
    assert_eq!(part1(&parsed), 1930);
}

#[test]
fn test_part2() {
    let parsed = parse(EXAMPLE_CONTRIVED);
    assert_eq!(part2(&parsed), 6 * 11 + 21 * 4);

    let parsed = parse(EXAMPLE_1);
    assert_eq!(part2(&parsed), 80);
}
