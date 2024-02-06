#![warn(clippy::pedantic, clippy::nursery)]

use atoi::atoi;

pub struct Game(u32, u32, u32);

pub fn parse(input: &str) -> Vec<Game> {
    // input.lines().collect()

    input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace().array_chunks::<2>().skip(1).fold(
                Game(0, 0, 0),
                |game, chunk| match chunk[1].as_bytes()[0] {
                    b'r' => Game(
                        game.0.max(atoi(chunk[0].as_bytes()).unwrap()),
                        game.1,
                        game.2,
                    ),
                    b'g' => Game(
                        game.0,
                        game.1.max(atoi(chunk[0].as_bytes()).unwrap()),
                        game.2,
                    ),
                    b'b' => Game(
                        game.0,
                        game.1,
                        game.2.max(atoi(chunk[0].as_bytes()).unwrap()),
                    ),
                    _ => unreachable!(),
                },
            )
        })
        .collect()
}

pub fn part1(input: &[Game]) -> usize {
    input
        .iter()
        .enumerate()
        .filter_map(|(index, game)| {
            if game.0 <= 12 && game.1 <= 13 && game.2 <= 14 {
                Some(index + 1)
            } else {
                None
            }
        })
        .sum()
}

pub fn part2(input: &[Game]) -> u32 {
    input.iter().map(|game| game.0 * game.1 * game.2).sum()
}

pub const fn verify() -> Option<(usize, u32)> {
    Some((2006, 84911))
}
