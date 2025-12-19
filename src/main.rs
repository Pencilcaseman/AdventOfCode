#![warn(clippy::pedantic, clippy::nursery)]

use std::path::PathBuf;

use clap::Parser;
use colored::Colorize;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    solutions: Vec<String>,

    /// More precise timing at the cost of runtime.
    ///
    /// Run each solution for n milliseconds and average the results
    ///
    /// *RUN BENCHMARKS FOR RELIABLE RESULTS*
    #[arg(short, long)]
    precise_timing: Option<u64>,
}

#[allow(clippy::cast_precision_loss)]
fn main() {
    let args = Args::parse();
    let target = &args.solutions;

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

        let mut part1 = 0;
        let mut part2 = 0;

        let elapsed = if let Some(time) = args.precise_timing {
            let target_time = std::time::Duration::from_millis(time);
            let mut samples = 0;

            let start = std::time::Instant::now();
            while start.elapsed() < target_time {
                (part1, part2) = std::hint::black_box(runner(&data));
                samples += 1;
            }
            start.elapsed() / samples
        } else {
            let start = std::time::Instant::now();
            (part1, part2) = std::hint::black_box(runner(&data));
            start.elapsed()
        };

        total += elapsed.as_micros();

        println!(
            "{0} {1} Day {2} {0}",
            "========".green().bold(),
            year.to_string().red().bold(),
            format!("{day:02}").red().bold()
        );

        println!("Part 1 : {}", part1.to_string().cyan());
        println!("Part 2 : {}", part2.to_string().cyan());
        println!("Elapsed: {}", format!("{elapsed:?}").cyan());
        println!();
    }

    println!("Total time: {}", format!("{}ms", (total as f64) / 1000.0).red());
}

struct Solution {
    pub year: u32,
    pub day: u32,
    pub runner: fn(&str) -> (i128, i128),
}

macro_rules! solution {
    ($year:tt, $day:tt) => {
        Solution {
            year: aoc::util::parse::parse_number(&stringify!($year)).unwrap(),
            day: aoc::util::parse::parse_number(&stringify!($day)).unwrap(),
            runner: |data: &str| {
                use aoc::$year::$day::*;

                let input = parse(data);
                (
                    i128::try_from(part1(&input)).unwrap(),
                    i128::try_from(part2(&input)).unwrap(),
                )
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
    vec![
        solution!(year2025, day01),
        solution!(year2025, day02),
        solution!(year2025, day03),
        solution!(year2025, day04),
        solution!(year2025, day05),
        solution!(year2025, day06),
        solution!(year2025, day07),
        solution!(year2025, day08),
        solution!(year2025, day09),
        solution!(year2025, day10),
        // solution!(year2025, day11),
        // solution!(year2025, day12),
    ]
}
