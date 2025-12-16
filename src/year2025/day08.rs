// TODO: usize vs u64?

use std::{
    cmp::{Ordering, min},
    collections::HashSet,
};

use itertools::Itertools;
use ndarray::Array2;
use rayon::prelude::*;

use crate::util::parse::ParseUnsigned;

type Input = (usize, usize);

pub fn parse(input: &str) -> Input {
    solve(ParseUnsigned::<usize>::new(input.bytes()).tuples().collect(), 1000)
}

pub fn part1(input: &Input) -> usize {
    input.0
}

pub fn part2(input: &Input) -> usize {
    input.1
}

pub fn solve(
    input: Vec<(usize, usize, usize)>,
    steps: usize,
) -> (usize, usize) {
    let mut pairwise_distances: Vec<_> = input
        .iter()
        .enumerate()
        .flat_map(|(i, p1)| {
            input
                .iter()
                .enumerate()
                .skip(i + 1)
                .map(move |(j, p2)| (dist(p1, p2), i, j))
        })
        .collect();

    pairwise_distances.par_sort_unstable_by(|a, b| a.0.total_cmp(&b.0));

    let mut dsu = DisjointSetUnion::new(input.len());

    let mut iter = pairwise_distances.into_iter();

    for (_, i, j) in iter.by_ref().take(steps) {
        dsu.merge(i, j);
    }

    let mut counts = dsu.counts().to_vec();
    counts.sort_unstable();
    let part1 = counts.into_iter().rev().take(3).product();
    let mut part2 = 0;

    while let Some((_, i, j)) = iter.next() {
        if dsu.merge(i, j) && dsu.counts().iter().max() == Some(&input.len()) {
            part2 = input[i].0 * input[j].0;
        }
    }

    (part1, part2)
}

#[derive(Debug)]
struct DisjointSetUnion {
    parents: Vec<usize>,
    ranks: Vec<usize>,
    counts: Vec<usize>,
}

impl DisjointSetUnion {
    fn new(n: usize) -> Self {
        Self {
            parents: (0..n).collect(),
            ranks: vec![1; n],
            counts: vec![1; n],
        }
    }

    fn parent(&mut self, mut i: usize) -> usize {
        while i != self.parents[i] {
            let p = self.parents[i];
            self.parents[i] = self.parents[p];
            i = self.parents[i];
        }
        i
    }

    fn merge(&mut self, i: usize, j: usize) -> bool {
        let parent_i = self.parent(i);
        let parent_j = self.parent(j);

        if parent_i == parent_j {
            // Already in the same circuit
            return false;
        }

        // Join the smaller tree onto the larger one
        match self.ranks[parent_i].cmp(&self.ranks[parent_j]) {
            Ordering::Less => {
                self.counts[parent_j] += self.counts[parent_i];
                self.parents[parent_i] = parent_j;
            }
            Ordering::Greater => {
                self.counts[parent_i] += self.counts[parent_j];
                self.parents[parent_j] = parent_i;
            }
            Ordering::Equal => {
                // Tree grows by 1
                self.counts[parent_i] += self.counts[parent_j];
                self.parents[parent_j] = parent_i;
                self.ranks[parent_i] += 1;
            }
        }

        true
    }

    fn counts(&self) -> &[usize] {
        &self.counts
    }
}

fn dist(a: &(usize, usize, usize), b: &(usize, usize, usize)) -> f64 {
    let ax = a.0 as f64;
    let ay = a.1 as f64;
    let az = a.2 as f64;

    let bx = b.0 as f64;
    let by = b.1 as f64;
    let bz = b.2 as f64;

    let dx = bx - ax;
    let dy = by - ay;
    let dz = bz - az;

    (dx * dx + dy * dy + dz * dz).cbrt()
}

// Answers for my input:
// Part 1: 24360
// Part 2: 2185817796
