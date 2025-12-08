use crate::util::parse::ParseUnsigned;

type Input = (u64, u64);

pub fn parse(input: &str) -> Input {
    let mut lines = input.lines().map(|s| s.bytes());

    let mut ranges = lines.by_ref().map_while(|s| {
        let mut bytes = ParseUnsigned::new(s);

        let first: u64 = bytes.next()?;
        let second: u64 = bytes.next()?;

        Some((first, second))
    });

    let mut active: Vec<bool> = Vec::with_capacity(192);
    let mut merged: Vec<(u64, u64)> = Vec::with_capacity(192);

    let mut current = ranges.next();

    while let Some(r) = current {
        let mut overlap = false;

        for i in 0..merged.len() {
            if !active[i] {
                continue;
            }

            let m = &merged[i];

            if r.0 <= m.1 && r.1 >= m.0 {
                overlap = true;
                active[i] = false;
                current = Some((r.0.min(m.0), r.1.max(m.1)));

                break;
            }
        }

        if !overlap {
            merged.push(r);
            active.push(true);
            current = ranges.next();
        }
    }

    let mut p1 = 0;
    for b in lines {
        let n = ParseUnsigned::new(b).next().unwrap();

        p1 += active
            .iter()
            .zip(merged.iter())
            .filter(|(a, _)| **a)
            .any(|(_, r)| r.0 <= n && n <= r.1) as u64;
    }

    let p2 = active
        .iter()
        .zip(merged.iter())
        .filter(|(a, _)| **a)
        .map(|(_, r)| r.1 - r.0 + 1)
        .sum();

    (p1, p2)
}

pub fn part1(input: &Input) -> u64 {
    input.0
}

pub fn part2(input: &Input) -> u64 {
    input.1
}

// Answers for my input:
// Part 1: 773
// Part 2: 332067203034711
