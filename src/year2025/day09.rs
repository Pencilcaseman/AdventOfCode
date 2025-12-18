//!
//! Almost entirely from maneatingape's solution... Absolutely amazing.
//! https://github.com/maneatingape/advent-of-code-rust/blob/main/src/year2025/day09.rs

use itertools::Itertools;

use crate::util::parse::ParseUnsigned;

type Input = Vec<(u64, u64)>;

pub fn parse(input: &str) -> Input {
    ParseUnsigned::<u64>::new(input.bytes()).tuples().collect()
}

pub fn part1(input: &Input) -> u64 {
    // dumb version

    input
        .iter()
        .enumerate()
        .flat_map(|(i, p0)| input.iter().skip(i + 1).map(|p1| area(p0, p1)))
        .max()
        .unwrap()
}

pub fn part2(input: &Input) -> u64 {
    let mut tmp = input.to_vec();
    tmp.sort_unstable_by_key(|&(row, col)| (row, col));

    debug_assert!(tmp.len().is_multiple_of(2), "Invalid input");

    let mut candidates = Vec::new();
    let mut vertical_edges = Vec::new();
    let mut intervals = Vec::new();

    let mut max_rect_area = 0;

    for ((row, col0), (row1, col1)) in tmp.iter().tuples() {
        // The input, sorted by (row, column), gives pairs of red tiles forming
        // horizontal edges on the same row with differing columns
        debug_assert_eq!(row, row1, "Invalid input");

        update_vertical_edge(*col0, &mut vertical_edges);
        update_vertical_edge(*col1, &mut vertical_edges);

        edges_to_intervals(&vertical_edges, &mut intervals);

        // Update largest rectangle
        // - Valid if candidate interval contains the current position
        // - Check if rectangle is larger than largest previously found
        [col0, col1].into_iter().for_each(|col| {
            candidates.iter().for_each(|c: &Candidate| {
                let a = area(&c.pos, &(*row, *col));

                if c.interval.contains(*col) && a > max_rect_area {
                    max_rect_area = a;
                }
            });
        });

        // Update candidates:
        // - If no interval contains the candidate, it is no longer valid
        candidates.retain_mut(|c| {
            if let Some(interval) =
                intervals.iter().find(|i| i.contains(c.pos.1))
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
        [col0, col1].into_iter().for_each(|&col| {
            // Only one interval can contain the current column
            if let Some(&interval) = intervals.iter().find(|i| i.contains(col))
            {
                candidates.push(Candidate { pos: (*row, col), interval })
            }
        });
    }

    max_rect_area
}

struct Candidate {
    pos: (u64, u64),
    interval: Interval,
}

#[derive(Debug, Clone, Copy)]
struct Interval(u64, u64);

impl Interval {
    fn contains(&self, col: u64) -> bool {
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

fn update_vertical_edge(red_x: u64, vertical_edges: &mut Vec<u64>) {
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

fn edges_to_intervals(edges: &[u64], intervals: &mut Vec<Interval>) {
    debug_assert!(edges.len().is_multiple_of(2));

    intervals.clear();
    intervals.extend(
        edges
            .iter()
            .tuples::<(&u64, &u64)>()
            .map(|(open, close)| Interval(*open, *close)),
    );
}

fn area(p0: &(u64, u64), p1: &(u64, u64)) -> u64 {
    (p0.0.abs_diff(p1.0) + 1) * (p0.1.abs_diff(p1.1) + 1)
}

// Answers for my input:
// Part 1: 4744899849
// Part 2: 1540192500
