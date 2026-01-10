use num::Integer;
use rayon::prelude::*;
use smallvec::SmallVec;

use crate::util::parse::ParseUnsigned;

type Input = Vec<MachineConfig>;

const MAX_PROBLEM_SIZE: usize = 16;
const MAX_FREE_VARS: usize = 4;

pub fn parse(input: &str) -> Input {
    input.lines().map_while(MachineConfig::new).collect()
}

pub fn part1(input: &Input) -> u32 {
    let mut working = WorkingSpace::default();

    input
        .iter()
        .map(|m| {
            solve_lights(m.target, &m.buttons, m.joltage.len(), &mut working)
        })
        .sum()
}

pub fn part2(input: &Input) -> i32 {
    input
        // .par_iter()
        .iter()
        .map(|machine_config| {
            full_solve(&machine_config.buttons, &machine_config.joltage)
                .unwrap()
        })
        .sum()
}

#[derive(Clone)]
pub struct MachineConfig {
    target: u32,
    buttons: Vec<u32>,
    joltage: Vec<i32>,
}

pub struct ProblemMatrix<const N: usize> {
    mat: [[i32; N]; N],
    rows: usize,
    cols: usize,
}

#[derive(Default)]
struct WorkingSpace {
    reduced: [u32; MAX_PROBLEM_SIZE],
    presses: [u32; MAX_PROBLEM_SIZE],
    pivots: [u32; MAX_PROBLEM_SIZE],
}

impl std::fmt::Debug for MachineConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MachineConfig {{ target: {:b}, buttons: (", self.target)?;
        for b in &self.buttons {
            write!(f, "{b:?} ")?;
        }
        write!(f, "), joltage: {:?} }}", self.joltage)
    }
}

impl MachineConfig {
    fn new(conf: &str) -> Option<MachineConfig> {
        let mut segments = conf.split_ascii_whitespace();

        let target = segments
            .next()
            .unwrap()
            .bytes()
            .skip(1)
            .enumerate()
            .fold(0, |toggle, (i, b)| toggle | ((b == b'#') as u32) << i);

        let joltage: Vec<_> =
            ParseUnsigned::<u32>::new(segments.next_back().unwrap().bytes())
                .map(|x| x as i32)
                .collect();

        let buttons: Vec<_> = segments
            .map(|s| {
                ParseUnsigned::<u8>::new(s.bytes())
                    .fold(0, |button, b| button | (1 << b))
            })
            .collect();

        Some(Self { target, buttons, joltage })
    }
}

fn solve_lights(
    target: u32,
    buttons: &[u32],
    width: usize,
    working: &mut WorkingSpace,
) -> u32 {
    let len = buttons.len();

    let reduced = &mut working.reduced[..len];
    let presses = &mut working.presses[..len];
    let pivots = &mut working.pivots[..len];

    // Initialize reduced and presses
    for (i, &button) in buttons.iter().enumerate() {
        reduced[i] = button;
        presses[i] = 1 << i;
    }

    // Convert reduced row echelon format with bitwise operations
    let mut rank = 0;
    for col in 0..width {
        // Find pivot
        let mask = 1 << col;

        let Some(pivot_idx) = (rank..len).find(|&i| reduced[i] & mask != 0)
        else {
            continue;
        };

        pivots[rank] = mask;

        reduced.swap(rank, pivot_idx);
        presses.swap(rank, pivot_idx);

        let pivot_r = reduced[rank];
        let pivot_p = presses[rank];

        // Skip j == rank
        // Doing it this way gives a 25% performance improvement, strangely
        for j in 0..rank {
            let m = 0u32.wrapping_sub(((reduced[j] & mask) != 0) as u32);
            reduced[j] ^= pivot_r & m;
            presses[j] ^= pivot_p & m;
        }
        for j in (rank + 1)..len {
            let m = 0u32.wrapping_sub(((reduced[j] & mask) != 0) as u32);
            reduced[j] ^= pivot_r & m;
            presses[j] ^= pivot_p & m;
        }

        rank += 1;
    }

    let nullity = len - rank;

    // Find particular solution
    let particular_solution = (0..rank).fold(0, |p_sol, row| {
        let m = 0u32.wrapping_sub((target & pivots[row] != 0) as u32);
        p_sol ^ presses[row] & m
    });

    // Try all combinations of free variables
    (0..(1 << nullity))
        .map(|null| {
            BitIterator::new(null)
                .fold(particular_solution, |p, i| p ^ presses[rank + i])
                .count_ones()
        })
        .min()
        .unwrap()
}

fn swap_rows<const N: usize>(mat: &mut [[i32; N]; N], i: usize, j: usize) {
    if i != j {
        mat.swap(i, j);
    }
}

fn scale_row<const N: usize>(mat: &mut [[i32; N]; N], i: usize, alpha: i32) {
    mat[i].iter_mut().for_each(|x| *x *= alpha);
}

fn rref<const N: usize>(mat: &mut ProblemMatrix<N>) {
    let rows = mat.rows;
    let cols = mat.cols;

    let mut rank = 0;
    let mut last = cols;

    while rank < rows && rank < last {
        // Pick the smallest coefficient to get better RREF reductions
        if let Some(pivot_row) = (rank..rows)
            .filter(|&r| mat.mat[r][rank] != 0)
            .min_by_key(|&r| mat.mat[r][rank].abs())
        {
            swap_rows(&mut mat.mat, pivot_row, rank);

            if mat.mat[rank][rank] < 0 {
                scale_row(&mut mat.mat, rank, -1);
            }

            for r in 0..rows {
                let coef = mat.mat[r][rank];
                let pivot_val = mat.mat[rank][rank];

                if r != rank && coef != 0 {
                    scale_row(&mut mat.mat, r, pivot_val);

                    (0..=cols).for_each(|c| {
                        mat.mat[r][c] -= mat.mat[rank][c] * coef;
                    });
                }
            }

            rank += 1;
        } else {
            // Move the `last` variable (last column) to the front instead of
            // skipping a column. This way all free variables are guaranteed to
            // be at the end of the matrix
            last -= 1;
            mat.mat[..rows + 1].iter_mut().for_each(|row| row.swap(rank, last));
        }

        // mat.mat.iter().for_each(|r| println!("{r:?}"));
        // println!();
    }

    // let nullity = cols - col;
    // println!("nullity = {nullity}");
    // todo!()
}

fn find_free_variables<const N: usize, const M: usize>(
    rref_mat: &ProblemMatrix<N>,
) -> SmallVec<usize, M> {
    let rows = rref_mat.rows;
    let cols = rref_mat.cols;

    let mut free = SmallVec::new();
    let mut col = 0;

    for row in &rref_mat.mat[..rows] {
        while col < cols && row[col] == 0 {
            free.push(col);
            col += 1
        }

        col += 1
    }

    free.extend(col..cols);

    free
}

fn solve_with_attempt<const N: usize, const M: usize>(
    rref_mat: &ProblemMatrix<N>,
    free_vars: &SmallVec<usize, M>,
    assignment: &SmallVec<i32, M>,
) -> Option<i32> {
    let rows = rref_mat.rows;
    let cols = rref_mat.cols;

    let mut row = 0;
    let mut col = 0;

    let mut total: i32 = assignment.iter().sum();

    while row < rows && col < cols {
        let mat_row = &rref_mat.mat[row];

        while col < cols && mat_row[col] == 0 {
            col += 1;
        }

        if col >= cols {
            continue;
        }

        let mut target = mat_row[cols];
        for i in 0..assignment.len() {
            target -= mat_row[free_vars[i]] * assignment[i];
        }

        if !target.is_multiple_of(&mat_row[col]) {
            return None;
        }

        let presses = target / mat_row[col];

        if presses < 0 {
            return None;
        }

        total += presses;

        row += 1
    }

    Some(total)
}

fn recurse<const N: usize, const M: usize>(
    rref_mat: &ProblemMatrix<N>,
    free_vars: &SmallVec<usize, M>,
    assignment: &mut SmallVec<i32, M>,
    depth: usize,
) -> Option<i32> {
    let rows = rref_mat.rows;
    let cols = rref_mat.cols;

    if assignment.len() == free_vars.len() {
        return solve_with_attempt(rref_mat, free_vars, assignment);
    }

    let free_col_idx = free_vars[depth];

    let mut lower_bound = 0;
    let mut upper_bound = rref_mat.mat[rows][free_vars[depth]];

    for row in &rref_mat.mat[..rows] {
        let mut target = row[cols];
        let coef = row[free_col_idx];

        if coef == 0 {
            continue;
        }

        let mut assigned_idx = 0;

        for c in 0..cols {
            if assigned_idx < depth && c == free_vars[assigned_idx] {
                target -=
                    row[free_vars[assigned_idx]] * assignment[assigned_idx];
                assigned_idx += 1;
                continue;
            }

            if c != free_col_idx && row[c] < 0 {
                target -= row[c] * rref_mat.mat[rows][c];
            }
        }

        if coef > 0 {
            upper_bound = upper_bound.min(target / coef);
        } else {
            lower_bound = lower_bound.max((target + coef + 1) / coef);
        }

        if upper_bound < lower_bound {
            return None;
        }
    }

    let mut best = i32::MAX;

    assignment.push(0);
    for b in lower_bound..=upper_bound {
        assignment[depth] = b;

        if let Some(new) = recurse(rref_mat, free_vars, assignment, depth + 1) {
            best = best.min(new);
        }
    }
    assignment.pop();

    if best == i32::MAX { None } else { Some(best) }
}

fn gen_matrix<const N: usize>(
    buttons: &[u32],
    joltage: &[i32],
) -> ProblemMatrix<N> {
    let rows = joltage.len();
    let cols = buttons.len();

    let mut mat = [[0; N]; N];

    for i in 0..rows {
        mat[i][cols] = joltage[i];
    }

    for col in 0..cols {
        let mut limit = i32::MAX;

        for toggle in BitIterator::new(buttons[col]) {
            mat[toggle][col] = 1i32;

            limit = limit.min(joltage[toggle]);
        }

        mat[rows][col] = limit;
    }

    ProblemMatrix { mat, rows, cols }
}

fn full_solve(buttons: &[u32], joltage: &[i32]) -> Option<i32> {
    let mut matrix = gen_matrix::<MAX_PROBLEM_SIZE>(buttons, joltage);

    rref(&mut matrix);
    let free_vars =
        find_free_variables::<MAX_PROBLEM_SIZE, MAX_FREE_VARS>(&matrix);

    let mut assignment = SmallVec::<i32, MAX_FREE_VARS>::new();
    recurse(&matrix, &free_vars, &mut assignment, 0)
}

// From https://github.com/maneatingape/advent-of-code-rust/blob/main/src/util/bitset.rs

struct BitIterator<T> {
    t: T,
}

impl<T> BitIterator<T> {
    fn new(t: T) -> Self
    where
        T: Copy,
    {
        Self { t }
    }
}

impl<T> std::iter::Iterator for BitIterator<T>
where
    T: num_traits::int::PrimInt,
{
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item>
    where
        T: num_traits::Zero + num_traits::One,
    {
        if self.t == T::zero() {
            None
        } else {
            let tz = self.t.trailing_zeros() as usize;
            self.t = self.t ^ (T::one() << tz);
            Some(tz)
        }
    }
}

// Answers for my input:
// Part 1: 449
// Part 2: 17848
