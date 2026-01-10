use num::Integer;
use rayon::prelude::*;
use smallvec::SmallVec;

use crate::util::{
    iter::HammingBitIter,
    parse::{ParseSigned, ParseUnsigned},
};

type Input = Vec<MachineConfig>;

const MAX_PROBLEM_SIZE: usize = 16;
const MAX_FREE_VARS: usize = 4;

pub fn parse(input: &str) -> Input {
    input.lines().map_while(MachineConfig::new).collect()
}

pub fn part1(input: &Input) -> u32 {
    let mut res = 0;

    for machine_config in input {
        let i_max = 1 << machine_config.buttons.len();
        let buttons = &machine_config.buttons;
        let target = machine_config.target;

        for i in HammingBitIter::new(i_max) {
            let mut val = 0;
            let mut temp = i;

            while temp != 0 {
                let idx = temp.trailing_zeros();
                val ^= buttons[idx as usize];
                temp &= temp.wrapping_sub(1);
            }

            if val == target {
                res += i.count_ones();
                break;
            }
        }
    }
    res
}

pub fn part2(input: &Input) -> i32 {
    input
        .par_iter()
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
        let segments: Vec<_> = conf.split_ascii_whitespace().collect();

        let number_seg_end = segments.len() - 1;

        let toggle_target = segments[0][1..]
            .bytes()
            .enumerate()
            .fold(0, |toggle, (i, b)| toggle | ((b == b'#') as u32) << i);

        let number_segs: Vec<_> = segments[1..number_seg_end]
            .iter()
            .map(|s| {
                ParseUnsigned::<u32>::new(s.bytes())
                    .fold(0, |button, b| button | (1 << b))
            })
            .collect();

        let joltage: Vec<_> =
            ParseUnsigned::<u32>::new(segments[number_seg_end].bytes())
                .map(|x| x as i32)
                .collect();

        Some(Self { target: toggle_target, buttons: number_segs, joltage })
    }
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

    let mut row = 0;
    let mut col = 0;

    while row < rows && col < cols {
        // Pick the smallest coefficient to get better RREF reductions
        let Some(pivot_row) = (row..rows)
            .filter(|&r| mat.mat[r][col] != 0)
            .min_by_key(|&r| mat.mat[r][col].abs())
        else {
            col += 1;
            continue;
        };

        swap_rows(&mut mat.mat, pivot_row, row);

        if mat.mat[row][col] < 0 {
            scale_row(&mut mat.mat, row, -1);
        }

        // Scale by LCM so all elements are divisible
        // Remove row from remaining rows if possible

        for r in 0..rows {
            let coef = mat.mat[r][col];
            let pivot_val = mat.mat[row][col];

            if r != row && coef != 0 {
                let lcm = pivot_val.lcm(&coef);

                let scale_dst = lcm / coef;
                let scale_src = lcm / pivot_val;

                (0..=cols).for_each(|c| {
                    mat.mat[r][c] =
                        mat.mat[r][c] * scale_dst - mat.mat[row][c] * scale_src;
                });
            }
        }

        row += 1;
    }
}

fn find_free_variables<const N: usize, const M: usize>(
    rref_mat: &ProblemMatrix<N>,
) -> SmallVec<usize, M> {
    let cols = rref_mat.cols;

    let mut free = SmallVec::new();
    let mut col = 0;

    for row in &rref_mat.mat {
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
    upper_bounds: &[i32],
    assignment: &mut SmallVec<i32, M>,
    depth: usize,
) -> Option<i32> {
    let cols = rref_mat.cols;

    if assignment.len() == free_vars.len() {
        return solve_with_attempt(rref_mat, free_vars, assignment);
    }

    let free_col_idx = free_vars[depth];

    let mut lower_bound = 0;
    let mut upper_bound = upper_bounds[free_vars[depth]];

    for row in rref_mat.mat {
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
                target -= row[c] * upper_bounds[c];
            }
        }

        if coef > 0 {
            upper_bound = upper_bound.min(target / coef);
        } else {
            // lower_bound = lower_bound.max(target / coef);
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

        if let Some(new) =
            recurse(rref_mat, free_vars, upper_bounds, assignment, depth + 1)
        {
            best = best.min(new);
        }
    }
    assignment.pop();

    if best == i32::MAX { None } else { Some(best) }
}

fn gen_matrix<const N: usize>(
    buttons: &[u32],
    joltage: &[i32],
) -> (ProblemMatrix<N>, Vec<i32>) {
    let rows = joltage.len();
    let cols = buttons.len();

    let mut mat = [[0; N]; N];

    let mut upper_bounds = vec![2048i32; cols];

    for i in 0..rows {
        mat[i][cols] = joltage[i];
    }

    for col in 0..cols {
        for toggle in BitIterator::new(buttons[col]) {
            mat[toggle][col] = 1i32;

            if (joltage[toggle]) < upper_bounds[col] {
                upper_bounds[col] = joltage[toggle];
            }
        }
    }

    let problem_matrix = ProblemMatrix { mat, rows, cols };

    (problem_matrix, upper_bounds)
}

fn full_solve(buttons: &[u32], joltage: &[i32]) -> Option<i32> {
    let (mut matrix, upper_bounds) =
        gen_matrix::<MAX_PROBLEM_SIZE>(buttons, joltage);

    rref(&mut matrix);
    let free_vars =
        find_free_variables::<MAX_PROBLEM_SIZE, MAX_FREE_VARS>(&matrix);

    let mut assignment = SmallVec::<i32, MAX_FREE_VARS>::new();
    recurse(&matrix, &free_vars, &upper_bounds, &mut assignment, 0)
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
