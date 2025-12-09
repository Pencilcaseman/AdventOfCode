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

    let mut merged: Vec<(u64, u64)> = Vec::with_capacity(192);

    let mut current = ranges.next();

    while let Some(r) = current {
        let mut overlap = false;

        for i in 0..merged.len() {
            let m = &merged[i];

            if r.0 <= m.1 && r.1 >= m.0 {
                overlap = true;
                current = Some((r.0.min(m.0), r.1.max(m.1)));
                merged.remove(i);

                break;
            }
        }

        if !overlap {
            let mut min = 0;
            let mut max = merged.len();

            while min < max {
                let mid = (min + max) / 2;

                if r.0 < merged[mid].0 {
                    max = mid;
                } else if r.0 > merged[mid].1 {
                    min = mid + 1;
                } else {
                    min = mid;
                    max = mid;
                }
            }

            merged.insert(min, r);

            current = ranges.next();
        }
    }

    let mut p1 = 0;

    for b in lines {
        let n = ParseUnsigned::new(b).next().unwrap();

        let mut min = 0;
        let mut max = merged.len();

        while min < max {
            let mid = (min + max) / 2;

            if n < merged[mid].0 {
                max = mid;
            } else if n > merged[mid].1 {
                min = mid + 1;
            } else {
                min = mid;
                max = mid;
            }
        }

        if min < merged.len() && merged[min].0 <= n && n <= merged[min].1 {
            p1 += 1;
        }
    }

    let p2 = merged.iter().map(|r| r.1 - r.0 + 1).sum();

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
