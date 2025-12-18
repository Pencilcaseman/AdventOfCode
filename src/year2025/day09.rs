//! transpose => 2x performance increase
//!
//! Almost entirely from maneatingape's solution... Absolutely amazing.
//! https://github.com/maneatingape/advent-of-code-rust/blob/main/src/year2025/day09.rs

use itertools::Itertools;

use crate::util::parse::ParseUnsigned;

type Input = Vec<(u32, u32)>;

pub fn parse(input: &str) -> Input {
    ParseUnsigned::<u32>::new(input.bytes()).tuples().collect()
}

pub fn part1(input: &Input) -> u64 {
    // dumb version
    input
        .iter()
        .enumerate()
        .flat_map(|(i, (p0_row, p0_col))| {
            input.iter().skip(i + 1).map(|(p1_row, p1_col)| {
                area(*p0_row, *p0_col, *p1_row, *p1_col)
            })
        })
        .max()
        .unwrap()
}

pub fn part2(input: &Input) -> u64 {
    let mut tmp = input.to_vec();
    tmp.sort_unstable_by_key(|(row, col)| (*col, *row));

    debug_assert!(tmp.len().is_multiple_of(2), "Invalid input");

    let mut candidates: Vec<Candidate> = Vec::with_capacity(512);
    let mut vertical_edges = Vec::with_capacity(4);
    let mut intervals = Vec::with_capacity(4);

    let mut max_rect_area = 0;

    for ((row0, col), (row1, col1)) in tmp.into_iter().tuples() {
        // The input, sorted by (row, column), gives pairs of red tiles forming
        // horizontal edges on the same row with differing columns
        debug_assert_eq!(col, col1, "Invalid input");

        for row in [row0, row1] {
            update_vertical_edge(row, &mut vertical_edges);
        }

        edges_to_intervals(&vertical_edges, &mut intervals);

        // Update largest rectangle
        // - Valid if candidate interval contains the current position
        // - Check if rectangle is larger than largest previously found
        for c in &candidates {
            for row in [row0, row1] {
                if c.interval.contains(row) {
                    max_rect_area =
                        max_rect_area.max(area(c.row, c.col, row, col));
                }
            }
        }

        // Update candidates:
        // - If no interval contains the candidate, it is no longer valid
        candidates.retain_mut(|c| {
            if let Some(interval) = intervals.iter().find(|i| i.contains(c.row))
            {
                c.interval = c.interval.intersection(*interval);
                true
            } else {
                false
            }
        });

        // Add new candidates
        // - If an existing interval contains this position, it is a candidate
        //   for a rectangle
        for row in [row0, row1] {
            // Only one interval can contain the current column
            if let Some(&interval) = intervals.iter().find(|i| i.contains(row))
            {
                candidates.push(Candidate { row, col, interval })
            }
        }
    }

    max_rect_area
}

struct Candidate {
    row: u32,
    col: u32,
    interval: Interval,
}

#[derive(Debug, Clone, Copy)]
struct Interval(u32, u32);

impl Interval {
    fn contains(&self, col: u32) -> bool {
        self.0 <= col && col <= self.1
    }

    fn overlaps(&self, other: &Self) -> bool {
        self.1 >= other.0 && self.0 <= other.1
    }

    fn intersection(self, other: Self) -> Self {
        debug_assert!(
            self.overlaps(&other),
            "Cannot intersect intervals that do not overlap"
        );

        Self(self.0.max(other.0), self.1.min(other.1))
    }
}

fn update_vertical_edge(red_x: u32, vertical_edges: &mut Vec<u32>) {
    match vertical_edges.binary_search(&red_x) {
        Ok(found) => {
            // Vertical edge already exists, so this red tile necessarily
            // closes it
            vertical_edges.remove(found);
        }
        Err(not_found) => {
            // Vertical edge does not exist, so create it
            vertical_edges.insert(not_found, red_x);
        }
    }
}

fn edges_to_intervals(edges: &[u32], intervals: &mut Vec<Interval>) {
    debug_assert!(edges.len().is_multiple_of(2));

    intervals.clear();
    intervals.extend(
        edges.iter().tuples().map(|(open, close)| Interval(*open, *close)),
    );
}

fn area(p0_row: u32, p0_col: u32, p1_row: u32, p1_col: u32) -> u64 {
    (p0_row.abs_diff(p1_row) + 1) as u64 * (p0_col.abs_diff(p1_col) + 1) as u64
}

// Answers for my input:
// Part 1: 4744899849
// Part 2: 1540192500
