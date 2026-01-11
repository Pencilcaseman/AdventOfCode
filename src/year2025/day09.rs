//! # Movie Theater
//!
//! This one had me stumped. All credit for this solution goes to
//! [maneatingape](https://github.com/maneatingape/advent-of-code-rust/blob/main/src/year2025/day09.rs)
//! for this solution. I have made a couple small improvements, but I could not
//! come up with this algorithm myself.
//!
//! Please check out maneatingape's solutions for a proper explanation of how
//! this works.
//!
//! Interestingly, transposing the input gives a 2x speedup for part 2 because
//! there is only ever a single interval to process; this is due to the input
//! being a circle with a vertical cutout in it.

use itertools::Itertools;

use crate::util::parse::ParseUnsigned;

type Input = Vec<(u32, u32)>;

pub fn parse(input: &str) -> Input {
    let mut tmp: Input =
        ParseUnsigned::<u32>::new(input.bytes()).tuples().collect();
    tmp.sort_unstable_by_key(|(row, col)| (*col, *row));
    tmp
}

pub fn part1(input: &Input) -> u64 {
    //
    // if for some (x, y) there exists (xx, yy)
    // s.t. xx < x and yy < y ==> (x, y) cannot be a corner
    //

    let (top_left, bottom_left) = corners(input.iter().copied());
    let (top_right, bottom_right) = corners(input.iter().copied().rev());

    let a0 = max_area(&top_left, &bottom_right);
    let a1 = max_area(&bottom_left, &top_right);
    a0.max(a1)
}

fn max_area(c0: &[(u32, u32)], c1: &[(u32, u32)]) -> u64 {
    let mut max = 0;
    for p0 in c0 {
        for p1 in c1 {
            let area = area(p0.0, p0.1, p1.0, p1.1);
            if area > max {
                max = area;
            }
        }
    }
    max
}

fn corners(
    iter: impl Iterator<Item = (u32, u32)>,
) -> (Vec<(u32, u32)>, Vec<(u32, u32)>) {
    let mut top_tiles = Vec::new();
    let mut bottom_tiles = Vec::new();

    let mut top_min_row = u32::MAX;
    let mut bottom_max_row = 0;

    let mut peek = iter.peekable();

    while let Some(top) = peek.next() {
        // Take all values in this column. There will always be at least one
        // value here
        let mut bottom = top.0;

        while let Some(n) = peek.next_if(|p| p.1 == top.1) {
            bottom = n.0;
        }

        let (min_row, max_row, col) =
            (top.0.min(bottom), top.0.max(bottom), top.1);

        // If this value is extreme in either way, take it

        if min_row < top_min_row {
            top_tiles.push((min_row, col));
            top_min_row = min_row;
        }

        if max_row > bottom_max_row {
            bottom_tiles.push((max_row, col));
            bottom_max_row = max_row;
        }
    }

    (top_tiles, bottom_tiles)
}

pub fn part2(input: &Input) -> u64 {
    debug_assert!(input.len().is_multiple_of(2), "Invalid input");

    let mut candidates: Vec<Candidate> = Vec::with_capacity(512);
    let mut vertical_edges = Vec::with_capacity(4);
    let mut intervals = Vec::with_capacity(4);

    let mut max_rect_area = 0;

    for (&(row0, col), &(row1, col1)) in input.iter().tuples() {
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
        //
        // Update candidates:
        // - If no interval contains the candidate, it is no longer valid
        candidates.retain_mut(|c| {
            for row in [row0, row1] {
                if c.interval.contains(row) {
                    max_rect_area =
                        max_rect_area.max(area(c.row, c.col, row, col));
                }
            }

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
