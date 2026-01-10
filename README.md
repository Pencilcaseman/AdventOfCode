# AdventOfCode

My Advent of Code Solutions in Rust. The code and algorithms used are optimised
to the best of my ability.

## Usage

For each question, put your input file in `input/year${YEAR}/day${DAY}.txt`
(e.g. `input/year2024/day01.txt`).

* To run every solution, run `cargo run`.
* To run a set of solutions, run `cargo run some_pattern`
  * e.g. `cargo run year2024` runs all solutions from 2024.
* For more information run `cargo run -- --help`

Additionally, replace `run` in the above commands with `test` or `bench` for
tests and benchmarks respectively.

**NOTE:** You may want to pass `--quiet` to the benchmarking program for
cleaner results. For example: `cargo bench -- year2025::day10 --quiet`.

## Solution Information

Many solutions do not have an in-depth explanation. In these cases, the
[descriptions in the commits for that question may contain more information](https://github.com/Pencilcaseman/AdventOfCode/commit/35e8650833bb372d2361bce7639c9f58abc75278)

