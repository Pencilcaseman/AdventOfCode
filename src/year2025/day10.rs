use num::Integer;
use rayon::prelude::*;

use crate::util::parse::{ParseSigned, ParseUnsigned};

type Input = Vec<MachineConfig>;
type ParseInt = u16;

const MAX_VAR_VAL: i32 = 2048;

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
            if let Some(res) =
                full_solve(&machine_config.buttons, &machine_config.joltage)
            {
                res.iter().copied().map(Fraction::to_int).sum::<i32>()
            } else {
                panic!("Failed to solve config")
            }
        })
        .sum()
}

#[derive(Clone)]
pub struct MachineConfig {
    target: u32,
    buttons: Vec<Vec<ParseInt>>,
    joltage: Vec<ParseInt>,
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
            if b == b'#' {
                toggle_target |= 1;
            }
            toggle_target <<= 1;
        }

        let mut number_segs: Vec<Vec<ParseInt>> = segments
            .map(|b| ParseUnsigned::<ParseInt>::new(b).collect())
            .collect();

        let joltage = number_segs.pop()?;

        Some(Self { target: toggle_target, buttons: number_segs, joltage })
    }
}

fn swap_rows(mat: &mut [Vec<Fraction>], i: usize, j: usize) {
    mat.swap(i, j);
}

fn scale_row(mat: &mut [Vec<Fraction>], i: usize, alpha: Fraction) {
    mat[i].iter_mut().for_each(|x| *x = *x * alpha);
}

fn add_scale_row(
    mat: &mut [Vec<Fraction>],
    src: usize,
    dst: usize,
    alpha: Fraction,
) {
    mat.get_disjoint_mut([src, dst]).iter_mut().for_each(|[src, dst]| {
        dst.iter_mut().zip(src.iter()).for_each(|(d, s)| *d = *d + *s * alpha);
    });
}

fn rref(mat: &mut [Vec<Fraction>]) {
    let rows = mat.len();
    let cols = mat[0].len() - 1;

    let mut pivot_row = 0;

    for col in 0..cols {
        if pivot_row >= rows {
            break;
        }

        let Some(pivot_candidate) =
            (pivot_row..rows).find(|&r| !mat[r][col].is_zero())
        else {
            continue;
        };

        swap_rows(mat, pivot_row, pivot_candidate);

        let pivot_val = mat[pivot_row][col];
        scale_row(mat, pivot_row, pivot_val.reciprocal());

        for r in 0..rows {
            if r != pivot_row {
                let factor = mat[r][col];
                add_scale_row(mat, pivot_row, r, -factor);
            }
        }

        pivot_row += 1
    }
}

fn find_free_variables(rref_mat: &[Vec<Fraction>]) -> Vec<usize> {
    let cols = rref_mat[0].len() - 1;

    let mut free = Vec::new();
    let mut col = 0;

    for row in rref_mat {
        while col < cols && row[col].is_zero() {
            free.push(col);
            col += 1
        }

        col += 1
    }

    free.extend(col..cols);

    free
}

fn solve_with_attempt(
    rref_mat: &[Vec<Fraction>],
    free_vars: &[usize],
    attempt: &[i32],
    best: i32,
) -> Option<Vec<Fraction>> {
    let vars = rref_mat[0].len() - 1;
    let mut solved = vec![Fraction::from_int(0); vars];

    let mut running_total = 0;

    for (i, x) in free_vars.iter().enumerate() {
        solved[*x] = Fraction::from_int(attempt[i]);
        running_total += attempt[i];
    }

    let mut col = 0;
    let mut free_idx = 0;

    for row in rref_mat {
        if running_total > best {
            return None;
        }

        while free_idx < free_vars.len() && col == free_vars[free_idx] {
            col += 1;
            free_idx += 1;
        }

        if col >= vars {
            break;
        }

        let mut target = row[vars];

        for b_idx in 0..free_vars.len() {
            target -=
                Fraction::from_int(attempt[b_idx]) * row[free_vars[b_idx]];
        }

        if target.is_negative() {
            return None;
        }

        solved[col] = target;
        running_total += target.to_int();
        col += 1;
    }

    Some(solved)
}

fn solve_recursive(
    rref_mat: &[Vec<Fraction>],
    max_vals: &[i32],
    free_vars: &[usize],
    attempt: &mut Vec<i32>,
    depth: usize,
    mut best: i32,
) -> Option<Vec<Fraction>> {
    if depth == free_vars.len() {
        return solve_with_attempt(rref_mat, free_vars, attempt, best);
    }

    // Find lower and upper bounds for free variable b_depth given the current
    // variable assignment.

    let num_vars = rref_mat[0].len() - 1;

    let mut high = Fraction::from_int(max_vals[free_vars[depth]].min(best));

    for row in rref_mat {
        let mut target = row[num_vars];

        // Undecided coefficients with opposite signs => no limit
        let mut seen_neg = false;

        let mut index = 0;
        let mut col = 0;

        while col < num_vars {
            if index < attempt.len() && col == free_vars[index] {
                target -=
                    Fraction::from_int(attempt[index]) * row[free_vars[index]];
                index += 1;
            } else if row[col].is_negative() {
                seen_neg = true;
                break;
            }

            col += 1;
        }

        let coef = row[free_vars[depth]];
        if !seen_neg && !coef.is_zero() {
            high = high.min(target / coef)
        }
    }

    let mut best_res = Vec::new();

    for free_var_val in 0..(high.to_int() + 1) {
        attempt.push(free_var_val);

        if let Some(solved) = solve_recursive(
            rref_mat,
            max_vals,
            free_vars,
            attempt,
            depth + 1,
            best,
        ) && solved.iter().all(|x| !x.is_negative() && x.is_int())
        {
            let sum = solved
                .iter()
                .fold(Fraction::from_int(0), |total, val| total + *val);
            if sum.to_int() < best {
                best_res = solved;
                best = sum.to_int();
            }
        }

        attempt.pop();
    }

    if best_res.is_empty() { None } else { Some(best_res) }
}

fn gen_matrix(
    buttons: &[Vec<ParseInt>],
    joltage: &[ParseInt],
) -> (Vec<Vec<Fraction>>, Vec<i32>) {
    let rows = joltage.len();
    let cols = buttons.len();

    let mut mat = vec![vec![Fraction::from_int(0); cols + 1]; rows];

    for col in 0..cols {
        for switch in &buttons[col] {
            mat[*switch as usize][col] = Fraction::from_int(1);
        }
    }

    for i in 0..rows {
        mat[i][cols] = Fraction::from_int(joltage[i] as i32);
    }

    let mut max_vals = vec![MAX_VAR_VAL; cols];

    for row in &mat {
        for i in 0..cols {
            if !row[i].is_zero() && row[cols].to_int() < max_vals[i] {
                max_vals[i] = row[cols].to_int();
            }
        }
    }

    (mat, max_vals)
}

fn full_solve(
    buttons: &[Vec<ParseInt>],
    joltage: &[ParseInt],
) -> Option<Vec<Fraction>> {
    let (mut matrix, max_vals) = gen_matrix(buttons, joltage);
    rref(&mut matrix);
    let free_vars = find_free_variables(&matrix);
    let mut attempt = Vec::new();
    solve_recursive(&matrix, &max_vals, &free_vars, &mut attempt, 0, i32::MAX)
}

#[derive(Clone, Copy)]
struct Fraction {
    num: i32,
    den: i32,
}

impl std::fmt::Debug for Fraction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.num, self.den)
    }
}

impl Fraction {
    fn new(mut num: i32, mut den: i32) -> Self {
        debug_assert_ne!(den, 0, "Division by zero");

        if num >= (1 << 13) || den >= (1 << 13) {
            let gcd = num::integer::gcd(num, den);
            num /= gcd;
            den /= gcd;
        }

        if den < 0 {
            num *= -1;
            den *= -1;
        }

        Self { num, den }
    }

    fn from_int(int: i32) -> Self {
        Self::new(int, 1)
    }

    fn to_int(self) -> i32 {
        self.num / self.den
    }

    fn reciprocal(&self) -> Self {
        Self::new(self.den, self.num)
    }

    fn is_int(&self) -> bool {
        self.num.is_multiple_of(&self.den)
    }

    fn is_zero(&self) -> bool {
        self.num == 0
    }

    fn is_negative(&self) -> bool {
        self.num < 0
    }
}

impl std::ops::Neg for Fraction {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.num, self.den)
    }
}

impl std::ops::Add for Fraction {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let num = self.num * rhs.den + self.den * rhs.num;
        let den = self.den * rhs.den;
        Self::new(num, den)
    }
}

impl std::ops::AddAssign for Fraction {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl std::ops::Sub for Fraction {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self + (-rhs)
    }
}

impl std::ops::SubAssign for Fraction {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl std::ops::Mul for Fraction {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let num = self.num * rhs.num;
        let den = self.den * rhs.den;
        Self::new(num, den)
    }
}

impl std::ops::Div for Fraction {
    type Output = Self;

    #[allow(clippy::suspicious_arithmetic_impl)]
    fn div(self, rhs: Self) -> Self::Output {
        self * rhs.reciprocal()
    }
}

impl std::cmp::PartialEq for Fraction {
    fn eq(&self, other: &Self) -> bool {
        (self.num * other.den) == (self.den * other.num)
    }
}

impl std::cmp::Eq for Fraction {}

#[allow(clippy::non_canonical_partial_ord_impl)]
impl std::cmp::PartialOrd for Fraction {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        (self.num * other.den).partial_cmp(&(self.den * other.num))
    }
}

impl std::cmp::Ord for Fraction {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.num * other.den).cmp(&(self.den * other.num))
    }
}

// Answers for my input:
// Part 1: 449
// Part 2: 17848
