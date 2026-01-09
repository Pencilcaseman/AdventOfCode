use ndarray::{Array2, s};

type Input = (Vec<(usize, usize)>, Array2<u8>);

const OFFSETS: [(usize, usize); 8] = [
    (usize::MAX, usize::MAX), // -1 -1
    (usize::MAX, 0),          // -1 0
    (usize::MAX, 1),          // -1 1
    (0, usize::MAX),          // 0 -1
    (0, 1),                   // 0 1
    (1, usize::MAX),          // 1 -1
    (1, 0),                   // 1 0
    (1, 1),                   // 1 1
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

    // First column
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

    // Middle columns
    for c in 1..(cols - 1) {
        if curr[c] != b'@' {
            continue;
        }

        let mut sum = (curr[c - 1] == b'@') as u8 + (curr[c + 1] == b'@') as u8;

        if HAS_ABOVE {
            let a = above.unwrap();
            sum += (a[c - 1] == b'@') as u8
                + (a[c] == b'@') as u8
                + (a[c + 1] == b'@') as u8;
        }

        if HAS_BELOW {
            let b = below.unwrap();
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
