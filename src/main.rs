#![warn(clippy::pedantic, clippy::nursery)]

use std::path::PathBuf;

use clap::Parser;
use colored::Colorize;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    solutions: Vec<String>,
}

#[allow(clippy::cast_precision_loss)]
fn main() {
    let target = Args::parse().solutions;

    let solutions = std::iter::empty()
        .chain(year2023())
        .chain(year2024())
        .chain(year2025())
        .filter(|s| {
            target.is_empty()
                || target.iter().any(|p| {
                    format!("year{}::day{:02}", s.year, s.day).contains(p)
                })
        });

    let mut total = 0;

    for Solution { year, day, runner } in solutions {
        let path: PathBuf = format!("input/year{year}/day{day:02}.txt").into();

        let Ok(data) = std::fs::read_to_string(&path) else {
            println!(
                "{}\n",
                format!("Failed to read input from {}", path.display())
                    .red()
                    .bold()
            );
            continue;
        };

        // Trim whitespace
        let data = data.trim_end().to_string();

        let start = std::time::Instant::now();
        let (part1, part2) = runner(&data);
        let elapsed = start.elapsed();

        total += elapsed.as_micros();

        println!(
            "{0} {1} Day {2} {0}",
            "========".green().bold(),
            year.to_string().red().bold(),
            format!("{day:02}").red().bold()
        );

        println!("Part 1 : {}", part1.cyan());
        println!("Part 2 : {}", part2.cyan());
        println!("Elapsed: {}", format!("{elapsed:?}").cyan());
        println!();
    }

    println!("Total time: {}", format!("{}ms", (total as f64) / 1000.0).red());
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
                (part1(&input).to_string(), part2(&input).to_string())
            },
        }
    };
}

fn year2023() -> Vec<Solution> {
    vec![
        solution!(year2023, day01),
        solution!(year2023, day02),
        solution!(year2023, day03),
        solution!(year2023, day04),
        solution!(year2023, day21),
    ]
}

fn year2024() -> Vec<Solution> {
    vec![
        solution!(year2024, day01),
        solution!(year2024, day02),
        solution!(year2024, day03),
        solution!(year2024, day04),
        solution!(year2024, day05),
        solution!(year2024, day06),
        solution!(year2024, day07),
        solution!(year2024, day08),
        solution!(year2024, day09),
        solution!(year2024, day10),
        solution!(year2024, day11),
        solution!(year2024, day12),
        solution!(year2024, day13),
        solution!(year2024, day14),
        solution!(year2024, day15),
    ]
}

fn year2025() -> Vec<Solution> {
    vec![solution!(year2025, day01)]
}
