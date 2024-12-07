#![warn(clippy::pedantic, clippy::nursery)]

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MapItem {
    Empty,
    Wall,
    Start,
    Seen((isize, isize)),
    Looper((isize, isize)),
}

type Map = Vec<Vec<MapItem>>;
type Input = (Map, (isize, isize));

// Rotates a direction 90 degrees clockwise
const fn rotate(dir: (isize, isize)) -> (isize, isize) {
    (dir.1, -dir.0)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TraceResult {
    Loop(usize),
    Exit(usize),
}

#[allow(
    clippy::cast_sign_loss,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::missing_panics_doc
)]
#[must_use]
pub fn trace_route(
    mut map: Map,
    mut pos: (isize, isize),
    mut dir: (isize, isize),
) -> TraceResult {
    let mut count = 0;

    loop {
        match map
            .get_mut(pos.0 as usize)
            .and_then(|row| row.get_mut(pos.1 as usize))
        {
            Some(MapItem::Wall) => {
                // Go backwards
                pos.0 -= dir.0;
                pos.1 -= dir.1;

                // Rotate
                dir = rotate(dir);

                // Go forwards
                pos.0 += dir.0;
                pos.1 += dir.1;
            }
            Some(item) => {
                // Empty, Start or Seen
                match item {
                    MapItem::Empty | MapItem::Start => {
                        count += 1;
                        pos.0 += dir.0;
                        pos.1 += dir.1;
                        *item = MapItem::Seen(dir);
                    }
                    MapItem::Looper(prev_dir) | MapItem::Seen(prev_dir) => {
                        if dir == *prev_dir {
                            // Found a loop
                            return TraceResult::Loop(count);
                        }

                        pos.0 += dir.0;
                        pos.1 += dir.1;
                    }
                    MapItem::Wall => unreachable!(),
                }
            }
            None => break,
        }
    }

    TraceResult::Exit(count)
}

#[allow(
    clippy::cast_sign_loss,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap
)]
#[must_use]
pub fn parse(input: &str) -> Input {
    let mut start = (0, 0);

    let map = input
        .as_bytes()
        .split(|&b| b == b'\n')
        .enumerate()
        .map(|(row, line)| {
            line.iter()
                .enumerate()
                .map(|(col, c)| match c {
                    b'.' => MapItem::Empty,
                    b'#' => MapItem::Wall,
                    b'^' => {
                        start = (row as isize, col as isize);
                        MapItem::Start
                    }
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();

    (map, start)
}

#[allow(
    clippy::cast_sign_loss,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::missing_panics_doc
)]
#[must_use]
pub fn part1(input: &Input) -> usize {
    match trace_route(input.0.clone(), input.1, (-1, 0)) {
        TraceResult::Loop(_) => 0,
        TraceResult::Exit(count) => count,
    }
}

#[allow(
    clippy::cast_sign_loss,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::missing_panics_doc
)]
#[must_use]
pub fn part2(input: &Input) -> usize {
    let mut map: Vec<Vec<MapItem>> = input.0.clone();
    let mut pos = input.1;
    let mut prev_pos;
    let mut dir = (-1, 0);
    let mut count = 0;

    loop {
        match map
            .get_mut(pos.0 as usize)
            .and_then(|row| row.get_mut(pos.1 as usize))
        {
            Some(MapItem::Wall) => {
                // Go backwards
                pos.0 -= dir.0;
                pos.1 -= dir.1;

                prev_pos = pos;

                // Rotate
                dir = rotate(dir);

                // Go forwards
                pos.0 += dir.0;
                pos.1 += dir.1;
            }
            Some(item) => {
                // Empty, Start or Seen
                match item {
                    MapItem::Empty | MapItem::Start => {
                        prev_pos = pos;

                        pos.0 += dir.0;
                        pos.1 += dir.1;
                        *item = MapItem::Seen(dir);
                    }
                    MapItem::Looper(_) | MapItem::Seen(_) => {
                        prev_pos = pos;

                        pos.0 += dir.0;
                        pos.1 += dir.1;
                    }
                    MapItem::Wall => unreachable!(),
                }
            }
            None => break,
        }

        // Redo the calculations but attempt to place a barrier at every
        // location. If we come across a cell that we've already seen in
        // the same direction, we know we've found a loop.

        if matches!(
            map[prev_pos.0 as usize][prev_pos.1 as usize],
            MapItem::Looper(_)
        ) {
            continue;
        }

        let MapItem::Seen(prev) = map[prev_pos.0 as usize][prev_pos.1 as usize]
        else {
            unreachable!()
        };

        map[prev_pos.0 as usize][prev_pos.1 as usize] = MapItem::Wall;

        if prev_pos != input.1
            && matches!(
                trace_route(map.clone(), prev_pos, dir),
                TraceResult::Loop(_)
            )
        {
            count += 1;
        }

        map[prev_pos.0 as usize][prev_pos.1 as usize] = MapItem::Looper(prev);
    }

    count
}

// For my input, the correct answer is:
// Part 1: 5199
// Part 2:
