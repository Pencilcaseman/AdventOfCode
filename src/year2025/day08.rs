//! # Playground
//!
//! This challenge is very easy to solve once you realize it maps exactly to
//! finding a minimum spanning tree (MST) of the junction locations.
//!
//! To find the MST efficiently, a disjoint set union (DSU) data structure is
//! used to efficiently check when a circuit contains a junction box. By
//! simultaneously tracking the size of each circuit, the solutions to part 1
//! and part 2drop out quite nicely.
//!
//! To improve the performance of the code, we split the input across multiple
//! threads and place edges in buckets according to their approximate length.
//! These buckets are then lazily flattened and sorted so they appear as a
//! contiguous array. This saves a significant amount of unnecessary
//! computation.
//!
//! A few optimizations to the DSU algorithm also provide a nice performance
//! improvement.

use itertools::Itertools;
use rayon::prelude::*;

use crate::util::parse::ParseUnsigned;

type Input = (u32, u32);

type BucketThreadEdge = Vec<Vec<Vec<(u64, u16, u16)>>>;

const NUM_BUCKETS: usize = 8;
const DIST_MAGNITUDE: u64 = 15_000 * 15_000;

pub fn parse(input: &str) -> Input {
    solve(ParseUnsigned::<u32>::new(input.bytes()).tuples().collect(), 1000)
}

pub fn part1(input: &Input) -> u32 {
    input.0
}

pub fn part2(input: &Input) -> u32 {
    input.1
}

pub fn parallel_load(input: &[(u32, u32, u32)]) -> BucketThreadEdge {
    let mut buckets = vec![vec![]; NUM_BUCKETS];
    let chunk_size = input.len() / rayon::current_num_threads();
    let indices: Vec<_> = (0..input.len()).collect();

    for b in indices
        .par_chunks(chunk_size)
        .map(|is| {
            let mut buckets = vec![vec![]; NUM_BUCKETS];

            for &i in is {
                for j in i + 1..input.len() {
                    let d = dist(&input[i], &input[j]);

                    let bucket =
                        (d / DIST_MAGNITUDE).min(NUM_BUCKETS as u64 - 1);
                    buckets[bucket as usize].push((d, i as u16, j as u16));
                }
            }

            buckets
        })
        .collect::<Vec<_>>()
    {
        buckets.iter_mut().zip(b).for_each(|(dst, b)| dst.push(b));
    }

    buckets
}

pub fn solve(input: Vec<(u32, u32, u32)>, steps: usize) -> (u32, u32) {
    let buckets = parallel_load(&input);
    let mut iter = flatten(&buckets);

    let mut dsu_parents: Vec<_> = (0..input.len()).collect();
    let mut dsu_counts = vec![1; input.len()];

    for (_, i, j) in iter.by_ref().take(steps) {
        let (i, j) = (i as usize, j as usize);
        dsu_merge(&mut dsu_parents, &mut dsu_counts, i, j);
    }

    let mut counts = dsu_counts.clone();
    counts.sort_unstable();
    let part1: u32 = counts.into_iter().rev().take(3).product();

    let mut part2 = 0;

    for (_, i, j) in iter {
        let (i, j) = (i as usize, j as usize);
        if dsu_merge(&mut dsu_parents, &mut dsu_counts, i, j)
            == input.len() as u32
        {
            part2 = input[i].0 * input[j].0;
            break;
        }
    }

    (part1, part2)
}

fn dist(a: &(u32, u32, u32), b: &(u32, u32, u32)) -> u64 {
    let dx = a.0.abs_diff(b.0) as u64;
    let dy = a.1.abs_diff(b.1) as u64;
    let dz = a.2.abs_diff(b.2) as u64;

    dx * dx + dy * dy + dz * dz
}

fn flatten(
    buckets: &BucketThreadEdge,
) -> impl Iterator<Item = (u64, u16, u16)> {
    buckets.iter().flat_map(|b| {
        let mut merged = b.concat();
        merged.sort_unstable_by_key(|v| v.0);
        merged
    })
}

fn dsu_find(parents: &mut [usize], mut i: usize) -> usize {
    while parents[i] != i {
        let parent = parents[i];
        parents[i] = parents[parent];
        i = parent;
    }

    i
}

fn dsu_merge(
    parents: &mut [usize],
    counts: &mut [u32],
    i: usize,
    j: usize,
) -> u32 {
    let mut parent_i = dsu_find(parents, i);
    let mut parent_j = dsu_find(parents, j);

    if parent_i != parent_j {
        if counts[parent_i] > counts[parent_j] {
            (parent_i, parent_j) = (parent_j, parent_i);
        }

        counts[parent_j] += counts[parent_i];
        parents[parent_i] = parent_j;
    }

    counts[parent_j]
}

// Answers for my input:
// Part 1: 24360
// Part 2: 2185817796
