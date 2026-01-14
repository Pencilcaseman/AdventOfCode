use std::simd::{
    Simd, SimdCast,
    cmp::{SimdPartialEq, SimdPartialOrd},
    i8x64,
    num::SimdInt,
    u8x64,
};

use ndarray::Array2;

type Input = (Vec<(usize, usize)>, Array2<u8>);

const OFFSETS: [(usize, usize); 8] = [
    (usize::MAX, usize::MAX), // -1 -1
    (usize::MAX, 0),          // -1  0
    (usize::MAX, 1),          // -1  1
    (0, usize::MAX),          //  0 -1
    (0, 1),                   //  0  1
    (1, usize::MAX),          //  1 -1
    (1, 0),                   //  1  0
    (1, 1),                   //  1  1
];

fn process_row<const HAS_ABOVE: bool, const HAS_BELOW: bool>(
    r: usize,
    rows: &[&[u8]],
    out_counts: &mut [u8],
    todo: &mut Vec<(usize, usize)>,
) {
    let cols = rows[0].len();

    let curr = rows[r];
    let above = if HAS_ABOVE { Some(rows[r - 1]) } else { None };
    let below = if HAS_BELOW { Some(rows[r + 1]) } else { None };

    let row_offset = r * cols;
    let counts = &mut out_counts[row_offset..row_offset + cols];

    // Starting at column 1 and ending at cols - 1
    let end_simd = if cols > 64 + 2 { cols - 64 - 1 } else { 0 };
    let at_splat = Simd::splat(b'@');

    // First column (scalar)
    if curr[0] == b'@' {
        let mut sum = (curr[1] == b'@') as u8;

        if HAS_ABOVE {
            let a = above.unwrap();
            sum += (a[0] == b'@') as u8 + (a[1] == b'@') as u8;
        }

        if HAS_BELOW {
            let b = below.unwrap();
            sum += (b[0] == b'@') as u8 + (b[1] == b'@') as u8;
        }

        counts[0] = sum;

        if sum < 4 {
            todo.push((r, 0));
        }
    }

    let mut c = 1;
    while c < end_simd {
        let chunk = u8x64::from_slice(&curr[c..]);
        let mask = chunk.simd_eq(at_splat);

        let mut acc = i8x64::splat(0);

        #[inline(always)]
        fn acc_row(acc: &mut i8x64, slice: &[u8], idx: usize, target: u8x64) {
            let l = Simd::from_slice(&slice[idx - 1..]);
            let m = Simd::from_slice(&slice[idx..]);
            let r = Simd::from_slice(&slice[idx + 1..]);

            // acc -= (-1) is equivalent to acc += 1
            *acc -= l.simd_eq(target).to_int();
            *acc -= m.simd_eq(target).to_int();
            *acc -= r.simd_eq(target).to_int();
        }

        let mid_l = Simd::from_slice(&curr[c - 1..]);
        let mid_r = Simd::from_slice(&curr[c + 1..]);
        acc -= mid_l.simd_eq(at_splat).to_int();
        acc -= mid_r.simd_eq(at_splat).to_int();

        if HAS_ABOVE {
            unsafe {
                acc_row(&mut acc, above.unwrap_unchecked(), c, at_splat);
            }
        }

        // -- Bottom Row Neighbors --
        if HAS_BELOW {
            unsafe {
                acc_row(&mut acc, below.unwrap_unchecked(), c, at_splat);
            }
        }

        let sums = acc.cast::<u8>();

        // Update counts where cell is '@'
        let old_vals = u8x64::from_slice(&counts[c..]);
        let new_vals = mask.select(sums, old_vals);
        new_vals.copy_to_slice(&mut counts[c..c + 64]);

        // Find todo values
        let mut todo_mask = (mask & sums.simd_lt(Simd::splat(4))).to_bitmask();
        while todo_mask != 0 {
            let i = todo_mask.trailing_zeros();
            todo.push((r, c + i as usize));
            todo_mask ^= 1 << i;
        }

        c += 64;
    }

    // Handle remaining columns before the end
    for c in c..(cols - 1) {
        if curr[c] != b'@' {
            continue;
        }

        let mut sum = (curr[c - 1] == b'@') as u8 + (curr[c + 1] == b'@') as u8;

        if HAS_ABOVE {
            let a = unsafe { above.unwrap_unchecked() };
            sum += (a[c - 1] == b'@') as u8
                + (a[c] == b'@') as u8
                + (a[c + 1] == b'@') as u8;
        }

        if HAS_BELOW {
            let b = unsafe { below.unwrap_unchecked() };
            sum += (b[c - 1] == b'@') as u8
                + (b[c] == b'@') as u8
                + (b[c + 1] == b'@') as u8;
        }

        counts[c] = sum;
        if sum < 4 {
            todo.push((r, c));
        }
    }

    // Final column
    let last = cols - 1;
    if curr[last] == b'@' {
        let mut sum = (curr[last - 1] == b'@') as u8;

        if HAS_ABOVE {
            let a = above.unwrap();
            sum += (a[last - 1] == b'@') as u8 + (a[last] == b'@') as u8;
        }
        if HAS_BELOW {
            let b = below.unwrap();
            sum += (b[last - 1] == b'@') as u8 + (b[last] == b'@') as u8;
        }

        counts[last] = sum;
        if sum < 4 {
            todo.push((r, last));
        }
    }
}

pub fn parse(input: &str) -> (Vec<(usize, usize)>, Array2<u8>) {
    let lines: Vec<&[u8]> = input.lines().map(|l| l.as_bytes()).collect();

    let rows = lines.len();
    let cols = lines[0].len();

    let mut count_data = vec![u8::MAX; rows * cols];
    let mut todo = Vec::new();

    process_row::<false, true>(0, &lines, &mut count_data, &mut todo);
    for r in 1..(rows - 1) {
        process_row::<true, true>(r, &lines, &mut count_data, &mut todo);
    }
    process_row::<true, false>(rows - 1, &lines, &mut count_data, &mut todo);

    let count_grid =
        unsafe { Array2::from_shape_vec_unchecked((rows, cols), count_data) };

    (todo, count_grid)
}

pub fn part1(input: &Input) -> usize {
    input.0.len()
}

pub fn part2(input: &Input) -> usize {
    let (mut todo, mut count_grid) = input.clone();
    let mut total_removed = 0;

    let dim = count_grid.dim();

    while let Some(pos) = todo.pop() {
        total_removed += 1;

        OFFSETS.into_iter().for_each(|offset| {
            let new =
                (pos.0.wrapping_add(offset.0), pos.1.wrapping_add(offset.1));

            if new.0 < dim.0 && new.1 < dim.1 {
                if count_grid[new] == 4 {
                    todo.push(new);
                }

                count_grid[new] -= 1;
            }
        });
    }

    total_removed
}

// Answers for my input:
// Part 1: 1553
// Part 2: 8442
