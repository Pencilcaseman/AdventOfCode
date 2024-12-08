#![warn(clippy::pedantic, clippy::nursery)]

use std::collections::HashSet;

use fxhash::FxBuildHasher;

type FastHashSet<T> = HashSet<T, FxBuildHasher>;

// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// pub enum MapItem {
//     Empty,
//     Wall,
//     Start,
//     Seen((isize, isize)),
//     Looper((isize, isize)),
// }
//
// type Map = Vec<Vec<MapItem>>;
// type Input = (Map, (isize, isize));
//
// // Rotates a direction 90 degrees clockwise
// const fn rotate(dir: (isize, isize)) -> (isize, isize) {
//     (dir.1, -dir.0)
// }
//
// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// pub enum TraceResult {
//     Loop(usize),
//     Exit(usize),
// }
//
// #[allow(
//     clippy::cast_sign_loss,
//     clippy::cast_possible_truncation,
//     clippy::cast_possible_wrap,
//     clippy::missing_panics_doc
// )]
// #[must_use]
// pub fn trace_route(
//     mut map: Map,
//     mut pos: (isize, isize),
//     mut dir: (isize, isize),
// ) -> TraceResult {
//     let mut count = 0;
//
//     loop {
//         match map
//             .get_mut(pos.0 as usize)
//             .and_then(|row| row.get_mut(pos.1 as usize))
//         {
//             Some(MapItem::Wall) => {
//                 // Go backwards
//                 pos.0 -= dir.0;
//                 pos.1 -= dir.1;
//
//                 // Rotate
//                 dir = rotate(dir);
//
//                 // Go forwards
//                 pos.0 += dir.0;
//                 pos.1 += dir.1;
//             }
//             Some(item) => {
//                 // Empty, Start or Seen
//                 match item {
//                     MapItem::Empty | MapItem::Start => {
//                         count += 1;
//                         pos.0 += dir.0;
//                         pos.1 += dir.1;
//                         *item = MapItem::Seen(dir);
//                     }
//                     MapItem::Looper(prev_dir) | MapItem::Seen(prev_dir) => {
//                         if dir == *prev_dir {
//                             // Found a loop
//                             return TraceResult::Loop(count);
//                         }
//
//                         pos.0 += dir.0;
//                         pos.1 += dir.1;
//                     }
//                     MapItem::Wall => unreachable!(),
//                 }
//             }
//             None => break,
//         }
//     }
//
//     TraceResult::Exit(count)
// }
//
// #[allow(
//     clippy::cast_sign_loss,
//     clippy::cast_possible_truncation,
//     clippy::cast_possible_wrap
// )]
// #[must_use]
// pub fn parse(input: &str) -> Input {
//     let mut start = (0, 0);
//
//     let map = input
//         .as_bytes()
//         .split(|&b| b == b'\n')
//         .enumerate()
//         .map(|(row, line)| {
//             line.iter()
//                 .enumerate()
//                 .map(|(col, c)| match c {
//                     b'.' => MapItem::Empty,
//                     b'#' => MapItem::Wall,
//                     b'^' => {
//                         start = (row as isize, col as isize);
//                         MapItem::Start
//                     }
//                     _ => unreachable!(),
//                 })
//                 .collect()
//         })
//         .collect();
//
//     (map, start)
// }
//
// #[allow(
//     clippy::cast_sign_loss,
//     clippy::cast_possible_truncation,
//     clippy::cast_possible_wrap,
//     clippy::missing_panics_doc
// )]
// #[must_use]
// pub fn part1(input: &Input) -> usize {
//     match trace_route(input.0.clone(), input.1, (-1, 0)) {
//         TraceResult::Loop(_) => 0,
//         TraceResult::Exit(count) => count,
//     }
// }
//
// #[allow(
//     clippy::cast_sign_loss,
//     clippy::cast_possible_truncation,
//     clippy::cast_possible_wrap,
//     clippy::missing_panics_doc
// )]
// #[must_use]
// pub fn part2(input: &Input) -> usize {
//     let mut map: Vec<Vec<MapItem>> = input.0.clone();
//     let mut pos = input.1;
//     let mut prev_pos;
//     let mut dir = (-1, 0);
//     let mut count = 0;
//
//     loop {
//         match map
//             .get_mut(pos.0 as usize)
//             .and_then(|row| row.get_mut(pos.1 as usize))
//         {
//             Some(MapItem::Wall) => {
//                 // Go backwards
//                 pos.0 -= dir.0;
//                 pos.1 -= dir.1;
//
//                 prev_pos = pos;
//
//                 // Rotate
//                 dir = rotate(dir);
//
//                 // Go forwards
//                 pos.0 += dir.0;
//                 pos.1 += dir.1;
//             }
//             Some(item) => {
//                 // Empty, Start or Seen
//                 match item {
//                     MapItem::Empty | MapItem::Start => {
//                         prev_pos = pos;
//
//                         pos.0 += dir.0;
//                         pos.1 += dir.1;
//                         *item = MapItem::Seen(dir);
//                     }
//                     MapItem::Looper(_) | MapItem::Seen(_) => {
//                         prev_pos = pos;
//
//                         pos.0 += dir.0;
//                         pos.1 += dir.1;
//                     }
//                     MapItem::Wall => unreachable!(),
//                 }
//             }
//             None => break,
//         }
//
//         // Redo the calculations but attempt to place a barrier at every
//         // location. If we come across a cell that we've already seen in
//         // the same direction, we know we've found a loop.
//
//         if matches!(
//             map[prev_pos.0 as usize][prev_pos.1 as usize],
//             MapItem::Looper(_)
//         ) {
//             continue;
//         }
//
//         let MapItem::Seen(prev) = map[prev_pos.0 as usize][prev_pos.1 as
// usize]         else {
//             unreachable!()
//         };
//
//         map[prev_pos.0 as usize][prev_pos.1 as usize] = MapItem::Wall;
//
//         if prev_pos != input.1
//             && matches!(
//                 trace_route(map.clone(), prev_pos, dir),
//                 TraceResult::Loop(_)
//             )
//         {
//             count += 1;
//         }
//
//         map[prev_pos.0 as usize][prev_pos.1 as usize] =
// MapItem::Looper(prev);     }
//
//     count
// }

// type Input = (usize, usize);

type Input = (Vec<Vec<u8>>, (usize, usize), (usize, usize));

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    const fn rotate(self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }
}

impl From<Direction> for (isize, isize) {
    fn from(dir: Direction) -> Self {
        match dir {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
        }
    }
}

impl From<(isize, isize)> for Direction {
    fn from(dir: (isize, isize)) -> Self {
        match dir {
            (-1, 0) => Self::Up,
            (1, 0) => Self::Down,
            (0, -1) => Self::Left,
            (0, 1) => Self::Right,
            _ => unreachable!(),
        }
    }
}

impl std::ops::Add<Direction> for (isize, isize) {
    type Output = (isize, isize);

    fn add(self, dir: Direction) -> Self::Output {
        match dir {
            Direction::Up => (self.0 - 1, self.1),
            Direction::Down => (self.0 + 1, self.1),
            Direction::Left => (self.0, self.1 - 1),
            Direction::Right => (self.0, self.1 + 1),
        }
    }
}

impl std::ops::AddAssign<Direction> for (isize, isize) {
    fn add_assign(&mut self, dir: Direction) {
        *self = *self + dir;
    }
}

impl std::ops::Sub<Direction> for (isize, isize) {
    type Output = (isize, isize);

    fn sub(self, dir: Direction) -> Self::Output {
        match dir {
            Direction::Up => (self.0 + 1, self.1),
            Direction::Down => (self.0 - 1, self.1),
            Direction::Left => (self.0, self.1 + 1),
            Direction::Right => (self.0, self.1 - 1),
        }
    }
}

impl std::ops::SubAssign<Direction> for (isize, isize) {
    fn sub_assign(&mut self, dir: Direction) {
        *self = *self - dir;
    }
}

#[allow(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_possible_wrap
)]
fn traverse(
    map: &[Vec<u8>],
    table: &mut [Vec<isize>],
    mut pos: (isize, isize),
    dir: Direction,
) {
    let rows = map.len();
    let cols = map[0].len();

    let mut count = 0;

    pos -= dir;

    while pos.0 >= 0
        && pos.0 < rows as isize
        && pos.1 >= 0
        && pos.1 < cols as isize
        && map[pos.0 as usize][pos.1 as usize] != b'#'
    {
        table[pos.0 as usize][pos.1 as usize] = count;
        count += 1;
        pos -= dir;
    }
}

impl std::ops::Index<Direction> for [Vec<Vec<isize>>; 4] {
    type Output = Vec<Vec<isize>>;

    fn index(&self, dir: Direction) -> &Self::Output {
        match dir {
            Direction::Up => &self[0],
            Direction::Down => &self[1],
            Direction::Left => &self[2],
            Direction::Right => &self[3],
        }
    }
}

impl std::ops::IndexMut<Direction> for [Vec<Vec<isize>>; 4] {
    fn index_mut(&mut self, dir: Direction) -> &mut Self::Output {
        match dir {
            Direction::Up => &mut self[0],
            Direction::Down => &mut self[1],
            Direction::Left => &mut self[2],
            Direction::Right => &mut self[3],
        }
    }
}

#[allow(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_possible_wrap
)]
pub fn parse(input: &str) -> Input {
    // let byte_map: Vec<Vec<u8>> =
    //     input.as_bytes().split(|&b| b ==
    // b'\n').map(<[u8]>::to_vec).collect();
    //
    // let rows = byte_map.len();
    // let cols = byte_map[0].len();
    //
    // let mut dir_steps: [Vec<Vec<isize>>; 4] = [
    //     vec![vec![-1isize; cols]; rows],
    //     vec![vec![-1isize; cols]; rows],
    //     vec![vec![-1isize; cols]; rows],
    //     vec![vec![-1isize; cols]; rows],
    // ];
    //
    // for row in 0..rows {
    //     for col in 0..cols {
    //         if byte_map[row][col] == b'#' {
    //             traverse(
    //                 &byte_map,
    //                 &mut dir_steps[Direction::Up],
    //                 (row as isize, col as isize),
    //                 Direction::Up,
    //             );
    //
    //             traverse(
    //                 &byte_map,
    //                 &mut dir_steps[Direction::Down],
    //                 (row as isize, col as isize),
    //                 Direction::Down,
    //             );
    //
    //             traverse(
    //                 &byte_map,
    //                 &mut dir_steps[Direction::Left],
    //                 (row as isize, col as isize),
    //                 Direction::Left,
    //             );
    //
    //             traverse(
    //                 &byte_map,
    //                 &mut dir_steps[Direction::Right],
    //                 (row as isize, col as isize),
    //                 Direction::Right,
    //             );
    //         }
    //     }
    // }
    //
    // (0, 0)

    // let grid: Vec<Vec<u8>> =
    //     input.as_bytes().split(|&b| b ==
    // b'\n').map(<[u8]>::to_vec).collect(); let start_idx =
    // input.bytes().position(|b| b == b'^').unwrap(); let cols =
    // input.bytes().position(|b| b == b'\n').unwrap() - 1;
    // let pos = (start_idx / cols, start_idx % cols);
    // let rows = input.as_bytes().len() / cols;
    // (grid, pos, (rows, cols))

    let mut grid = Vec::with_capacity(128);
    let mut row = Vec::new();
    let mut pos = (0, 0);

    let mut row_idx = 0;
    let mut col_idx = 0;

    for byte in input.bytes() {
        match byte {
            b'\n' => {
                grid.push(row.clone());
                row.clear();

                row_idx += 1;
                col_idx = 0;
            }
            b'^' => {
                pos = (row_idx, col_idx);
                row.push(b'^');
            }
            other => {
                col_idx += 1;
                row.push(other);
            }
        }
    }

    if !row.is_empty() {
        grid.push(row);
    }

    let rows = grid.len();
    let cols = grid[0].len();

    (grid, pos, (rows, cols))
}

#[allow(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_possible_wrap,
    clippy::match_on_vec_items
)]
#[must_use]
pub fn part1((grid, pos, (rows, cols)): &Input) -> usize {
    let mut grid = grid.clone();
    let mut pos = (pos.0 as isize, pos.1 as isize);
    let mut dir = Direction::Up;

    let mut count = 1; // Include the start position

    loop {
        let next = pos + dir;

        if next.0 < 0
            || next.0 >= *rows as isize
            || next.1 < 0
            || next.1 >= *cols as isize
        {
            break;
        }

        match grid[next.0 as usize][next.1 as usize] {
            b'#' => {
                dir = dir.rotate();
                continue;
            }
            b'.' => {
                grid[next.0 as usize][next.1 as usize] = b'^';
                count += 1;
            }
            _ => {}
        }

        pos = next;
    }

    count
}

#[allow(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_possible_wrap,
    clippy::match_on_vec_items
)]
#[must_use]
pub fn part2((grid, pos, (rows, cols)): &Input) -> usize {
    let mut grid = grid.clone();
    let mut pos = (pos.0 as isize, pos.1 as isize);
    let mut dir = Direction::Up;
    let mut path = Vec::with_capacity(4096);

    loop {
        let next = pos + dir;

        if next.0 < 0
            || next.0 >= *rows as isize
            || next.1 < 0
            || next.1 >= *cols as isize
        {
            break;
        }

        match grid[next.0 as usize][next.1 as usize] {
            b'#' => {
                dir = dir.rotate();
                continue;
            }
            b'.' => {
                grid[next.0 as usize][next.1 as usize] = b'^';
            }
            _ => {}
        }

        path.push((pos, dir));
        pos = next;
    }

    path.iter()
        .filter(|(pos, dir)| is_loop(&grid, (*rows, *cols), *pos, *dir))
        .count()
}

#[allow(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_possible_wrap,
    clippy::match_on_vec_items
)]
#[must_use]
pub fn is_loop(
    grid: &[Vec<u8>],
    size: (usize, usize),
    mut pos: (isize, isize),
    mut dir: Direction,
) -> bool {
    let mut grid = grid.to_vec();
    let mut seen = FastHashSet::with_capacity_and_hasher(
        size.0 * size.1,
        FxBuildHasher::default(),
    );

    // Mark the starting position as an obstacle
    grid[pos.0 as usize][pos.1 as usize] = b'#';

    loop {
        // If we've seen this position before in this direction, we've found a
        // loop
        if !seen.insert((pos, dir)) {
            return true;
        }

        let next = pos + dir;

        if next.0 < 0
            || next.0 >= size.0 as isize
            || next.1 < 0
            || next.1 >= size.1 as isize
        {
            return false;
        }

        match grid[next.0 as usize][next.1 as usize] {
            b'#' => {
                dir = dir.rotate();
                continue;
            }
            b'.' => {
                grid[next.0 as usize][next.1 as usize] = b'^';
            }
            _ => {}
        }

        pos = next;
    }
}

// For my input, the correct answer is:
// Part 1: 5199
// Part 2: 1915
