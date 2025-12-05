//! Advent of Code Solutions in Rust
//!
//! My solutions are, to the best of my ability, optimised both algorithmically
//! and implementation-wise. I've also tried to explain the solutions as best I
//! can, so anyone struggling can get a better idea of what's going on.

#![feature(binary_heap_into_iter_sorted)]
#![feature(iter_array_chunks)]
#![feature(int_roundings)]

/// Utility modules to handle common tasks
pub mod util {
    pub mod integer;
    pub mod parse;
    pub mod point;
}

/// Solutions for year 2023
pub mod year2023 {
    pub mod day01;
    pub mod day02;
    pub mod day03;
    pub mod day04;
    pub mod day21;
    pub mod day22;
}

pub mod year2024 {
    pub mod day01;
    pub mod day02;
    pub mod day03;
    pub mod day04;
    pub mod day05;
    pub mod day06;
    pub mod day07;
    pub mod day08;
    pub mod day09;
    pub mod day10;
    pub mod day11;
    pub mod day12;
    pub mod day13;
    pub mod day14;
    pub mod day15;
}

pub mod year2025 {
    pub mod day01;
    pub mod day02;
    pub mod day03;
    pub mod day04;
}
