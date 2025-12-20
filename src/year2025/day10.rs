use rayon::prelude::*;

use crate::util::parse::ParseUnsigned;

type Input = Vec<MachineConfig>;

pub fn parse(input: &str) -> Input {
    input.lines().map_while(|l| parse_machine_config(l)).collect()
}

pub fn part1(input: &Input) -> u32 {
    input
        .par_iter()
        .map(|machine_config| {
            let num_buttons = machine_config.buttons.len();

            let mut best = u32::MAX;

            for presses in 0u32..(1 << num_buttons) {
                let mut config = 0;

                for bit in 0..num_buttons {
                    if (presses & (1 << bit)) != 0 {
                        config ^= machine_config.buttons[bit];
                    }
                }

                if config == machine_config.target
                    && presses.count_ones() < best.count_ones()
                {
                    best = presses;
                }
            }

            best.count_ones()
        })
        .sum()
}

pub fn part2(input: &Input) -> u32 {
    0
}

pub struct MachineConfig {
    target: u32,
    buttons: Vec<u32>,
    joltage: Vec<u32>,
}

impl std::fmt::Debug for MachineConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MachineConfig {{ target: {:b}, buttons: (", self.target)?;
        for b in &self.buttons {
            write!(f, "{b:b} ")?;
        }
        write!(f, "), joltage: {:?} }}", self.joltage)
    }
}

fn parse_machine_config(conf: &str) -> Option<MachineConfig> {
    let mut iter = conf.bytes();

    // Take first [
    let _ = iter.next()?;

    // Exract target toggle states
    let mut target = 0;
    let mut target_iter = iter.by_ref().enumerate();
    while let Some((i, b)) = target_iter.next()
        && b != b']'
    {
        if b == b'#' {
            target |= 1 << i;
        }
    }

    // Take space
    let _ = iter.next()?;

    // Extract button toggle options
    let mut buttons = Vec::new();

    while let Some(b) = iter.next()
        && b == b'('
    {
        // Take digits until ')' is found
        // Toggle options are always 0 <= b <= 9
        let mut button = 0;
        while let Some(b) = iter.next()
            && b != b' '
        {
            button |= 1 << (b - b'0');

            // Take comma or space
            let _ = iter.next()?;
        }

        buttons.push(button);
    }

    let joltage: Vec<_> = ParseUnsigned::<u32>::new(iter).collect();

    Some(MachineConfig { target, buttons, joltage })
}

// Answers for my input:
// Part 1: 449
// Part 2:
