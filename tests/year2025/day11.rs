use aoc::year2025::day11::*;

const EXAMPLE_PART1: &str = "\
aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out";

const EXAMPLE_PART2: &str = "\
svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out";

#[test]
fn test_part1() {
    let input = parse(EXAMPLE_PART1);
    assert_eq!(part1(&input), 5);
}

#[test]
fn test_part2() {
    let input = parse(EXAMPLE_PART2);
    assert_eq!(part2(&input), 2);
}
