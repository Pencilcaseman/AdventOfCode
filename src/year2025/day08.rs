// TODO: usize vs u64?

use std::cmp::Ordering;

use itertools::Itertools;
use rayon::prelude::*;

use crate::util::parse::ParseUnsigned;

type Input = (u32, u32);

const CUTOFF_LIMIT: usize = 20;

pub fn parse(input: &str) -> Input {
    solve(ParseUnsigned::<usize>::new(input.bytes()).tuples().collect(), 1000)
}

pub fn part1(input: &Input) -> u32 {
    input.0
}

pub fn part2(input: &Input) -> u32 {
    input.1
}

fn dist(a: &(usize, usize, usize), b: &(usize, usize, usize)) -> usize {
    let dx = a.0.abs_diff(b.0);
    let dy = a.1.abs_diff(b.1);
    let dz = a.2.abs_diff(b.2);

    dx * dx + dy * dy + dz * dz
}

pub fn solve(input: Vec<(usize, usize, usize)>, steps: usize) -> (u32, u32) {
    let mut pairwise_distances = Vec::new();
    let mut cutoff = usize::MAX;

    for i in 0..input.len() {
        for j in (i + 1)..input.len() {
            let d = dist(&input[i], &input[j]);
            if d < cutoff {
                pairwise_distances.push((d, i, j));
            }
        }

        if i == CUTOFF_LIMIT {
            pairwise_distances.par_sort_unstable_by_key(|x| x.0);
            cutoff = pairwise_distances[steps - 1].0;
        }
    }

    pairwise_distances.par_sort_unstable_by_key(|x| x.0);

    let mut dsu = DisjointSetUnion::new(input.len());

    let mut iter = pairwise_distances.into_iter();

    for (_, i, j) in iter.by_ref().take(steps) {
        dsu.merge(i, j);
    }

    let mut counts = dsu.counts().to_vec();
    counts.sort_unstable();
    let part1 = counts.into_iter().rev().take(3).product();
    let mut part2 = 0;

    for (_, i, j) in iter {
        if dsu.merge(i, j)
            && *dsu.counts().iter().max().unwrap() == input.len() as u32
        {
            part2 = input[i].0 * input[j].0;
        }
    }

    (part1, part2 as u32)
}

#[derive(Debug)]
struct DisjointSetUnion {
    parents: Vec<usize>,
    ranks: Vec<u8>,
    counts: Vec<u32>,
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

    fn counts(&self) -> &[u32] {
        &self.counts
    }
}

// Answers for my input:
// Part 1: 24360
// Part 2: 2185817796
