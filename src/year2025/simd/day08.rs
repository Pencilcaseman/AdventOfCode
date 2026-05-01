//! Janky solution that avoids the bucketing of the original solution and
//! instead cuts off overly large distances. This will not work in the general
//! case, and is not guaranteed to work on all inputs. That said, it is much
//! faster.
//!
//! Please create a PR/issue if you know of a better solution

use itertools::Itertools;
use rayon::prelude::*;

use crate::util::parse::ParseUnsigned;

type Input = (u32, u32);

const CUTOFF: u64 = 15000 * 15000;

pub fn parse(input: &str) -> Input {
    solve(ParseUnsigned::<u32>::new(input.bytes()).tuples().collect(), 1000)
}

pub fn part1(input: &Input) -> u32 {
    input.0
}

pub fn part2(input: &Input) -> u32 {
    input.1
}

pub fn parallel_load(input: &[(u32, u32, u32)]) -> Vec<(u64, u16, u16)> {
    let chunk_size = input.len() / rayon::current_num_threads();
    let num_chunks = input.len().div_ceil(chunk_size);

    let res = (0..num_chunks)
        .into_par_iter()
        .map(|chunk_id| {
            let start = chunk_id * chunk_size;
            let end = ((chunk_id + 1) * chunk_size).min(input.len());

            let mut bucket = vec![];

            for i in start..end {
                for j in i + 1..input.len() {
                    let d = dist(&input[i], &input[j]);

                    if d > CUTOFF {
                        continue;
                    }

                    bucket.push((d, i as u16, j as u16));
                }
            }

            bucket
        })
        .collect::<Vec<_>>();

    let mut res = res.concat();
    res.sort_unstable_by_key(|x| x.0);

    res
}

pub fn solve(input: Vec<(u32, u32, u32)>, steps: usize) -> (u32, u32) {
    let bucket = parallel_load(&input);
    let mut iter = bucket.into_iter();

    let mut dsu_parents: Vec<_> = (0..input.len() as u16).collect();
    let mut dsu_counts = vec![1; input.len()];

    for (_, i, j) in iter.by_ref().take(steps) {
        dsu_merge(&mut dsu_parents, &mut dsu_counts, i, j);
    }

    let mut counts = dsu_counts.clone();
    counts.par_sort_unstable();
    let part1: u32 = counts.into_iter().rev().take(3).product();

    let mut part2 = 0;

    for (_, i, j) in iter {
        if dsu_merge(&mut dsu_parents, &mut dsu_counts, i, j)
            == input.len() as u32
        {
            part2 = input[i as usize].0 * input[j as usize].0;
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

// fn flatten(buckets: &ThreadEdge) -> impl Iterator<Item = (u64, u16, u16)> {
//     buckets.iter().flat_map(|b| {
//         let mut merged = b.concat();
//         merged.sort_unstable_by_key(|v| v.0);
//         merged
//     })
// }

fn dsu_find(parents: &mut [u16], mut i: u16) -> u16 {
    while parents[i as usize] != i {
        let parent = parents[i as usize];
        parents[i as usize] = parents[parent as usize];
        i = parent;
    }

    i
}

fn dsu_merge(parents: &mut [u16], counts: &mut [u32], i: u16, j: u16) -> u32 {
    let mut parent_i = dsu_find(parents, i);
    let mut parent_j = dsu_find(parents, j);

    if parent_i != parent_j {
        if counts[parent_i as usize] > counts[parent_j as usize] {
            (parent_i, parent_j) = (parent_j, parent_i);
        }

        counts[parent_j as usize] += counts[parent_i as usize];
        parents[parent_i as usize] = parent_j;
    }

    counts[parent_j as usize]
}

// Answers for my input:
// Part 1: 24360
// Part 2: 2185817796
