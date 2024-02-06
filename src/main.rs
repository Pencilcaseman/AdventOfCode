#![warn(clippy::pedantic, clippy::nursery)]

use clap::Parser;
use colored::*;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The solutions to run
    solutions: Vec<String>,
}

fn main() {
    let target = Args::parse().solutions;

    let solutions = std::iter::empty().chain(year2023()).filter(|s| {
        target.is_empty()
            || target
                .iter()
                .any(|p| format!("year{}::day{:02}", s.year, s.day).contains(p))
    });

    for Solution { year, day, runner } in solutions {
        let path: PathBuf = format!("input/year{year}/day{day:02}.txt").into();

        let Ok(data) = std::fs::read_to_string(&path) else {
            eprintln!("Failed to read input from {}", path.display());
            continue;
        };

        let start = std::time::Instant::now();
        let (part1, part2) = runner(&data);
        let elapsed = start.elapsed();

        println!(
            "{0} {1} Day {2:02} {0}",
            "=====".green().bold(),
            year.to_string().red().bold(),
            day.to_string().red().bold()
        );
        println!("Part 1 : {}", part1.purple());
        println!("Part 2 : {}", part2.purple());
        println!("Elapsed: {}", format!("{elapsed:?}").red());
    }
}

struct Solution {
    pub year: u32,
    pub day: u32,
    pub runner: fn(&str) -> (String, String),
}

macro_rules! solution {
    ($year:tt, $day:tt) => {
        Solution {
            year: aoc::util::parse::parse_number(&stringify!($year)).unwrap(),
            day: aoc::util::parse::parse_number(&stringify!($day)).unwrap(),
            runner: |data: &str| {
                use aoc::$year::$day::*;

                let input = parse(data);
                let part1 = part1(&input);
                let part2 = part2(&input);

                if let Some(correct) = verify() {
                    assert_eq!(part1, correct.0);
                    assert_eq!(part2, correct.1);
                }

                (part1.to_string(), part2.to_string())
            },
        }
    };
}

fn year2023() -> Vec<Solution> {
    vec![solution!(year2023, day01)]
}
