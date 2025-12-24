use num::Integer;
use rayon::prelude::*;

use crate::util::parse::{ParseSigned, ParseUnsigned};

type Input = Vec<MachineConfig>;

pub fn parse(input: &str) -> Input {
    input.lines().map_while(MachineConfig::new).collect()
}

// pub fn part1(input: &Input) -> u32 {
//     input
//         .par_iter()
//         .map(|machine_config| {
//             let num_buttons = machine_config.buttons.len();
//
//             let mut best = u32::MAX;
//
//             for presses in 0u32..(1 << num_buttons) {
//                 let mut config = 0;
//
//                 for bit in 0..num_buttons {
//                     if (presses & (1 << bit)) != 0 {
//                         config ^= machine_config.buttons[bit];
//                     }
//                 }
//
//                 if config == machine_config.target
//                     && presses.count_ones() < best.count_ones()
//                 {
//                     best = presses;
//                 }
//             }
//
//             best.count_ones()
//         })
//         .sum()
// }

pub fn part1(input: &Input) -> u32 {
    0
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
    buttons: Vec<Vec<i32>>,
    joltage: Vec<i32>,
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
        let mut segments = conf.split(' ').map(|s| s.bytes());

        // Parse the toggle states
        let mut toggle_target = 0;
        let toggle_bytes = segments.next()?;
        for b in toggle_bytes {
            toggle_target <<= 1;
            if b == b'#' {
                toggle_target |= 1;
            }
        }

        let mut number_segs: Vec<Vec<i32>> =
            segments.map(|b| ParseSigned::<i32>::new(b).collect()).collect();

        let joltage = number_segs.pop()?;

        Some(Self { target: toggle_target, buttons: number_segs, joltage })
    }
}

fn swap_rows(mat: &mut [Vec<i32>], i: usize, j: usize) {
    if i != j {
        mat.swap(i, j);
    }
}

fn scale_row(mat: &mut [Vec<i32>], i: usize, alpha: i32) {
    mat[i].iter_mut().for_each(|x| *x *= alpha);
}

fn rref(mat: &mut [Vec<i32>]) {
    let rows = mat.len();
    let cols = mat[0].len() - 1;

    let mut row = 0;
    let mut col = 0;

    while row < rows && col < cols {
        let Some(pivot_row) = (row..rows).find(|&r| mat[r][col] != 0) else {
            col += 1;
            continue;
        };

        swap_rows(mat, pivot_row, row);

        if mat[row][col] < 0 {
            scale_row(mat, row, -1);
        }

        // Scale by LCM so all elements are divisible
        // Remove row from remaining rows if possible

        for r in 0..rows {
            let coef = mat[r][col];
            let pivot_val = mat[row][col];

            if r != row && coef != 0 {
                let lcm = pivot_val.lcm(&coef).abs();

                let scale_dst = lcm / coef.abs();
                let scale_src = lcm / pivot_val.abs() * coef.signum();

                for c in 0..=cols {
                    mat[r][c] = mat[r][c] * scale_dst - mat[row][c] * scale_src;
                }
            }
        }

        row += 1;
    }
}

fn find_free_variables(rref_mat: &[Vec<i32>]) -> Vec<usize> {
    let cols = rref_mat[0].len() - 1;

    let mut free = Vec::new();
    let mut col = 0;

    for row in rref_mat {
        while col < cols && row[col] == 0 {
            free.push(col);
            col += 1
        }

        col += 1
    }

    free.extend(col..cols);

    free
}

fn solve_with_attempt(
    rref_mat: &[Vec<i32>],
    free_vars: &[usize],
    assignment: &[i32],
) -> Option<i32> {
    let rows = rref_mat.len();
    let cols = rref_mat[0].len() - 1;

    let mut row = 0;
    let mut col = 0;

    let mut total: i32 = assignment.iter().sum();

    while row < rows && col < cols {
        while col < cols && rref_mat[row][col] == 0 {
            col += 1;
        }

        if col >= cols {
            continue;
        }

        let target = rref_mat[row][cols]
            - free_vars
                .iter()
                .zip(assignment)
                .map(|(&var, val)| rref_mat[row][var] * val)
                .sum::<i32>();

        if !target.is_multiple_of(&rref_mat[row][col]) {
            return None;
        }

        let presses = target / rref_mat[row][col];

        if presses < 0 {
            return None;
        }

        total += presses;

        row += 1
    }

    Some(total)
}

fn recurse(
    rref_mat: &[Vec<i32>],
    free_vars: &[usize],
    lower_bounds: &[i32],
    upper_bounds: &[i32],
    assignment: &mut Vec<i32>,
    depth: usize,
) -> Option<i32> {
    let cols = rref_mat[0].len() - 1;

    if assignment.len() == free_vars.len() {
        return solve_with_attempt(rref_mat, free_vars, assignment);
    }

    let free_col_idx = free_vars[depth];

    let mut lower_bound = lower_bounds[free_vars[depth]];
    let mut upper_bound = upper_bounds[free_vars[depth]];

    for row in rref_mat {
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

            if c != free_col_idx {
                if row[c] > 0 {
                    target -= row[c] * lower_bounds[c];
                } else {
                    target -= row[c] * upper_bounds[c];
                }
            }
        }

        if coef > 0 {
            upper_bound = upper_bound.min(target / coef);
        } else {
            lower_bound = lower_bound.max(target / coef);
        }

        if upper_bound < lower_bound {
            return None;
        }
    }

    let mut best = i32::MAX;

    for b in lower_bound..=upper_bound {
        assignment.push(b);

        if let Some(new) = recurse(
            rref_mat,
            free_vars,
            lower_bounds,
            upper_bounds,
            assignment,
            depth + 1,
        ) {
            best = best.min(new);
        }

        assignment.pop();
    }

    if best == i32::MAX { None } else { Some(best) }
}

fn gen_matrix(
    buttons: &[Vec<i32>],
    joltage: &[i32],
) -> (Vec<Vec<i32>>, Vec<i32>, Vec<i32>) {
    let rows = joltage.len();
    let cols = buttons.len();

    let mut mat = vec![vec![0; cols + 1]; rows];
    let mut lower_bounds = vec![0; cols];
    let mut upper_bounds = vec![2048i32; cols];

    for col in 0..cols {
        for &toggle in &buttons[col] {
            mat[toggle as usize][col] = 1i32;

            if (joltage[toggle as usize] as i32) < upper_bounds[col] {
                upper_bounds[col] = joltage[toggle as usize] as i32;
            }
        }
    }

    for i in 0..rows {
        mat[i][cols] = joltage[i] as i32;
    }

    (mat, lower_bounds, upper_bounds)
}

fn full_solve(buttons: &[Vec<i32>], joltage: &[i32]) -> Option<i32> {
    let (mut matrix, lower_bounds, upper_bounds) = gen_matrix(buttons, joltage);
    rref(&mut matrix);
    let free_vars = find_free_variables(&matrix);

    let mut assignment = Vec::new();
    recurse(
        &matrix,
        &free_vars,
        &lower_bounds,
        &upper_bounds,
        &mut assignment,
        0,
    )
}

// Answers for my input:
// Part 1: 449
// Part 2: 17848
