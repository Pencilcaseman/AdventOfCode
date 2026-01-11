use std::array::from_fn;

use num::Integer;
use rayon::prelude::*;

use crate::util::parse::ParseUnsigned;

type Input = Vec<MachineConfig>;

const MAX_PROBLEM_SIZE: usize = 14;

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

fn rref<const N: usize>(mat: &mut ProblemMatrix<N>) -> ResultThing<N> {
    let ProblemMatrix { mat, rows, cols } = mat;
    let rows = *rows;
    let cols = *cols;

    let mut rank = 0;
    let mut last = cols;

    while rank < rows && rank < last {
        // Pick the smallest coefficient to keep coefficients small
        if let Some(pivot_row) = (rank..rows)
            .filter(|&r| mat[r][rank] != 0)
            .min_by_key(|&r| mat[r][rank].abs())
        {
            mat.swap(pivot_row, rank);

            if mat[rank][rank] < 0 {
                mat[rank][rank..cols + 1].iter_mut().for_each(|x| *x *= -1);
            }

            for r in 0..rows {
                let coef = mat[r][rank];
                let pivot_val = mat[rank][rank];

                if r != rank && coef != 0 {
                    (0..cols + 1).for_each(|c| {
                        mat[r][c] = mat[r][c] * pivot_val - mat[rank][c] * coef;
                    });
                }
            }

            rank += 1;
        } else {
            // Move the `last` variable (last column) to the front instead of
            // skipping a column. This way all free variables are guaranteed to
            // be at the end of the matrix
            last -= 1;
            mat[..rows + 1].iter_mut().for_each(|row| row.swap(rank, last));
        }
    }

    // Pivot coefficients are not necessarily 1, so find LCM and scale rows
    // accordingly
    let lcm = (0..rank).fold(1, |lcm, r| mat[r][r].lcm(&lcm));
    for (pivot_idx, row) in mat[..rank].iter_mut().enumerate() {
        let scale = lcm / row[pivot_idx];
        row[pivot_idx..cols + 1].iter_mut().for_each(|v| *v *= scale);
    }

    let nullity = cols - rank;

    let rhs: [_; N] = from_fn(|row| mat[row][cols]);
    let particular_solution: i32 = rhs[..rank].iter().sum();

    let free_vars: Vec<_> = (0..nullity)
        .map(|null| {
            let vector = from_fn(|row| mat[row][rank + null]);
            let limit = mat[rows][rank + null];
            let cost = lcm - vector[..rank].iter().sum::<i32>();

            FreeVariable::<N> { vector, limit, cost }
        })
        .collect();

    ResultThing { rank, lcm, particular_solution, rhs, free_vars }
}

// TODO: Come up with a better name
struct ResultThing<const N: usize> {
    rank: usize,
    lcm: i32,
    particular_solution: i32,
    rhs: [i32; N],
    free_vars: Vec<FreeVariable<N>>,
}

struct FreeVariable<const N: usize> {
    vector: [i32; N],
    limit: i32,
    cost: i32,
}

fn recurse<const N: usize>(
    free_vars: &[FreeVariable<N>],
    rank: usize,
    lcm: i32,
    mut rhs: [i32; N],
    mut remaining: u32,
    presses: i32,
) -> Option<i32> {
    if free_vars.is_empty() {
        return Some(presses / lcm);
    }

    let mut tmp_rhs = rhs;

    // Negative coefficients allow for infinite solutions. Fortunately, the
    // variables are bounded so we can offset by the maximum possible value and
    // only search that space
    for i in BitIterator::new(remaining) {
        let var = &free_vars[i];
        tmp_rhs[..rank].iter_mut().zip(var.vector).for_each(|(r, v)| {
            if v < 0 {
                *r -= v * var.limit;
            }
        })
    }

    // Find variable with largest range
    let mut best_lower = 0;
    let mut best_upper = i32::MIN;
    let mut best_idx = usize::MAX;
    let mut smallest_size = i32::MAX;

    for i in BitIterator::new(remaining) {
        let var = &free_vars[i];

        let mut lower = 0;
        let mut upper = var.limit;

        for (&v, rhs) in var.vector[..rank].iter().zip(&tmp_rhs) {
            if v > 0 {
                upper = upper.min(rhs / v);
            } else if v < 0 {
                // Attempting to solve for this var, so undo the offset by
                // `lower` from earlier
                let tmp_rhs_val = rhs + v * var.limit;
                lower = lower.max((tmp_rhs_val + v + 1) / v);
            }
        }

        // Inclusive -- [lower, upper]
        let size = upper - lower + 1;

        if size > 0 && size < smallest_size {
            smallest_size = size;
            best_lower = lower;
            best_upper = upper;
            best_idx = i;
        }
    }

    if best_idx == usize::MAX {
        return None;
    }

    // Remove selected variable
    remaining ^= 1 << best_idx;

    let best_var = &free_vars[best_idx];

    if remaining != 0 {
        // Same as above -- reduce search space by lower bound
        rhs[..rank]
            .iter_mut()
            .zip(best_var.vector)
            .for_each(|(r, v)| *r -= v * best_lower);

        (best_lower..=best_upper)
            .filter_map(|f| {
                let total = recurse(
                    free_vars,
                    rank,
                    lcm,
                    rhs,
                    remaining,
                    presses + f * best_var.cost,
                );

                // f is 0, 1, 2, ...
                // so rhs decreases by 1 * v each time
                rhs[..rank]
                    .iter_mut()
                    .zip(&best_var.vector)
                    .for_each(|(r, v)| *r -= v);

                total
            })
            .min()
    } else {
        // Solve
        (best_lower..=best_upper)
            .filter_map(|f| {
                let total_presses = presses + f * best_var.cost;

                // Check if result is integer
                let is_integer = rhs[..rank]
                    .iter()
                    .zip(best_var.vector)
                    .all(|(r, v)| (r - f * v).is_multiple_of(&lcm));

                if is_integer { Some(total_presses / lcm) } else { None }
            })
            .min()
    }
}

fn gen_matrix<const N: usize>(
    buttons: &[u32],
    joltage: &[i32],
) -> ProblemMatrix<N> {
    let rows = joltage.len();
    let cols = buttons.len();

    let mut mat = [[0; N]; N];

    for col in 0..cols {
        let mut limit = i32::MAX;

        for toggle in BitIterator::new(buttons[col]) {
            mat[toggle][col] = 1i32;
            limit = limit.min(joltage[toggle]);
        }

        mat[rows][col] = limit;
    }

    for i in 0..rows {
        mat[i][cols] = joltage[i];
    }

    ProblemMatrix { mat, rows, cols }
}

fn full_solve(buttons: &[u32], joltage: &[i32]) -> Option<i32> {
    let mut matrix = gen_matrix::<MAX_PROBLEM_SIZE>(buttons, joltage);

    let ResultThing { rank, lcm, particular_solution, free_vars, rhs } =
        rref(&mut matrix);

    let remaining = (1 << free_vars.len()) - 1;
    recurse(&free_vars, rank, lcm, rhs, remaining, particular_solution)
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
