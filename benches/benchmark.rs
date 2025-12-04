use criterion::*;

macro_rules! bench {
    ($year:tt, $day:tt) => {
        pub mod $day {
            use std::{path::PathBuf, sync::OnceLock};

            use aoc::$year::$day::*;
            use criterion::*;

            /// Load the input data from the file only once, and store it
            /// statically for future use.
            fn load_once() -> &'static str {
                static DATA: OnceLock<String> = OnceLock::new();
                DATA.get_or_init(|| {
                    let path: PathBuf = format!(
                        "input/{}/{}.txt",
                        stringify!($year),
                        stringify!($day)
                    )
                    .into();

                    let Ok(data) = std::fs::read_to_string(&path) else {
                        panic!("Failed to read input from {}", path.display());
                    };

                    data.trim_end().to_string()
                })
            }

            fn name() -> String {
                format!("{}::{}", stringify!($year), stringify!($day))
            }

            pub fn bench_parse<
                WallTime: criterion::measurement::Measurement,
            >(
                group: &mut BenchmarkGroup<'_, WallTime>,
            ) {
                group.bench_function("parse", |b| {
                    let data = load_once();
                    b.iter(|| parse(data))
                });
            }

            pub fn bench_part1<
                WallTime: criterion::measurement::Measurement,
            >(
                group: &mut BenchmarkGroup<'_, WallTime>,
            ) {
                group.bench_function("part1", |b| {
                    let data = load_once();
                    let input = parse(data);
                    b.iter(|| part1(&input))
                });
            }

            pub fn bench_part2<
                WallTime: criterion::measurement::Measurement,
            >(
                group: &mut BenchmarkGroup<'_, WallTime>,
            ) {
                group.bench_function("part2", |b| {
                    let data = load_once();
                    let input = parse(data);
                    b.iter(|| part2(&input))
                });
            }

            pub fn bench(c: &mut Criterion) {
                let mut group = c.benchmark_group(&name());
                let _ = load_once();
                bench_parse(&mut group);
                bench_part1(&mut group);
                bench_part2(&mut group);
                group.finish();
            }
        }
    };
}

pub mod year2023 {
    bench!(year2023, day01);
    bench!(year2023, day02);
    bench!(year2023, day03);
    bench!(year2023, day04);
    bench!(year2023, day21);
    // bench!(year2023, day22);
}

pub mod year2024 {
    bench!(year2024, day01);
    bench!(year2024, day02);
    bench!(year2024, day03);
    bench!(year2024, day04);
    bench!(year2024, day05);
    bench!(year2024, day06);
    bench!(year2024, day07);
    bench!(year2024, day08);
    bench!(year2024, day09);
    bench!(year2024, day10);
    bench!(year2024, day11);
    bench!(year2024, day12);
    bench!(year2024, day13);
    bench!(year2024, day14);
    bench!(year2024, day15);
}

pub mod year2025 {
    bench!(year2025, day01);
    bench!(year2025, day02);
}

pub fn criterion_benchmark(c: &mut Criterion) {
    year2023::day01::bench(c);
    year2023::day02::bench(c);
    year2023::day03::bench(c);
    year2023::day04::bench(c);
    year2023::day21::bench(c);
    // year2023::day22::bench(c);

    year2024::day01::bench(c);
    year2024::day02::bench(c);
    year2024::day03::bench(c);
    year2024::day04::bench(c);
    year2024::day05::bench(c);
    year2024::day06::bench(c);
    year2024::day07::bench(c);
    year2024::day08::bench(c);
    year2024::day09::bench(c);
    year2024::day10::bench(c);
    year2024::day11::bench(c);
    year2024::day12::bench(c);
    year2024::day13::bench(c);
    year2024::day14::bench(c);
    year2024::day15::bench(c);

    year2025::day01::bench(c);
    year2025::day02::bench(c);
    year2025::day03::bench(c);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
