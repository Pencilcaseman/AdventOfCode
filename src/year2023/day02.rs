#![warn(clippy::pedantic, clippy::nursery)]

use atoi::atoi;

pub struct Game(u32, u32, u32);

pub fn parse(input: &str) -> Vec<Game> {
    // Split each line into chunks of size 2 (["5", "red"], ["3", "green"], etc.). Skip the first
    // one, as it's the Game ID. For each one, match against the first letter, as that's sufficient
    // to determine which colour it represents -- then just find the maximum value for each color.
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
    // If the number of reds is greater than 12, the number of greens is greater than 13 or the
    // number of blues is greater than 14, we reject that game. Otherwise, sum the indices.
    // Note that we add one to the indices to account for 1-based indexing.
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
    // Multiply the maximum values for each color and sum them all up.
    input.iter().map(|game| game.0 * game.1 * game.2).sum()
}

// For my input, the correct answer is:
// Part 1: 2006
// Part 2: 84911
