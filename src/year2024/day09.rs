#![warn(clippy::pedantic, clippy::nursery)]

use std::{cmp::Reverse, collections::BinaryHeap};

type Input = Vec<usize>;

#[must_use]
pub fn parse(input: &str) -> Input {
    input.bytes().map(|b| (b - b'0') as usize).collect()
}

/// Returns the partial sum for a section
///
/// For example, the input `12345` looks like
/// 0..111....33333
///    ^^^
///
/// Given id=1, start=3, count=3, it returns 1 * 3 + 1 * 4 + 1 * 5 = 12
///
/// A derivation is given below:
///
/// ```math
/// let x be the id
/// let s be the starting index (start)
/// let n be the end element (count - 1)
///
/// We have sum = s*x + (s+1)*x + (s+2)*x) + ... + (s+n)*x
///             = sx + sx+x + sx+2x + ... + sx+nx
///             = s(n+1)x + 0x + x + 2x + ... + nx
///             = s(n+1)x + (n(n+1)/2)x
///             = snx + sx + (n(n+1)/2)x
///             = x(s(n+1)+(n(n+1)/2))
///
/// Since n will always be in [0, 9], we can avoid calculating (n(n+1)/2) and
/// instead use a lookup table: [0, 1, 3, 6, 10, 15, 21, 28, 36, 45]
///
/// Hence, we have sum = x(s(n+1)+LOOKUP[n])
///
/// Refactoring to use the count directly:
/// LOOKUP = [0, 0, 1, 3, 6, 10, 15, 21, 28, 36]
/// sum = x(s * count + LOOKUP[count])
/// ```
const fn partial_checksum(id: usize, start: usize, count: usize) -> usize {
    const LOOKUP: [usize; 10] = [0, 0, 1, 3, 6, 10, 15, 21, 28, 36];
    id * (start * count + LOOKUP[count])
}

#[must_use]
pub fn part1(input: &Input) -> usize {
    let mut left = 0;
    let mut right = input.len() - 1;
    let mut needed = input[right];
    let mut block_idx = 0;
    let mut sum = 0;

    while left < right {
        sum += partial_checksum(left / 2, block_idx, input[left]);
        block_idx += input[left];
        let mut available = input[left + 1];
        left += 2;

        // Go backwards until no more space is available in the current empty
        // section
        while available > 0 {
            // If we don't need anything more, move further back
            if needed == 0 {
                if left >= right {
                    break;
                }

                right -= 2;
                needed = input[right];
            }

            let amt = std::cmp::min(available, needed);
            sum += partial_checksum(right / 2, block_idx, amt);

            block_idx += amt;
            available -= amt;
            needed -= amt;
        }
    }

    // Clean up any remaining file blocks
    sum += partial_checksum(right / 2, block_idx, needed);

    sum
}

#[must_use]
pub fn part2(input: &Input) -> usize {
    let mut heaps: [BinaryHeap<Reverse<usize>>; 10] = Default::default();
    // let mut sum = 0;

    // let mut sim_index = 0;
    // let mut real_index = 0;
    // while real_index + 1 < input.len() {
    //     // Compute the current sum and then subtract from it to avoid
    //     // recalculating
    //     sum += partial_checksum(real_index / 2, sim_index,
    // input[real_index]);
    //
    //     sim_indices[real_index] = sim_index;
    //     sim_index += input[real_index];
    //     sim_indices[real_index + 1] = sim_index;
    //
    //     let gap_size = input[real_index + 1];
    //     heaps[gap_size].push(Reverse(sim_index));
    //     sim_index += gap_size;
    //
    //     real_index += 2;
    // }

    let mut idx = 0;
    for (index, &size) in input.iter().enumerate() {
        if index % 2 == 1 {
            heaps[size].push(Reverse(idx));
        }

        idx += size;
    }

    // Get the last element as well
    // sim_index += input[input.len() - 1];
    // sim_indices[input.len() - 1] = sim_index;
    // sum += partial_checksum(input.len() / 2, sim_index, input[input.len() -
    // 1]);

    // println!("Initial sum: {sum}");

    // println!("{heaps:?}");

    // while right > 0 {
    //     let needed = input[right];
    //
    //     // Find a gap at least as large as the required size
    //
    //     'outer: for size in needed..10 {
    //         // println!(
    //         //     "Checking size {size}: {:?} | {}",
    //         //     heaps[size].peek(),
    //         //     sim_indices[right]
    //         // );
    //
    //         match heaps[size].peek() {
    //             Some(gap_index) if gap_index.0 < sim_indices[right] => {
    //                 // Move the block
    //                 let Some(gap_index) = heaps[size].pop() else {
    //                     unreachable!()
    //                 };
    //
    //                 // println!(
    //                 //     "Move {needed} elements from {} to {}",
    //                 //     sim_indices[right], gap_index.0
    //                 // );
    //
    //                 sum += partial_checksum(right / 2, gap_index.0, needed);
    //                 sum -=
    //                     partial_checksum(right / 2, sim_indices[right],
    // needed);
    //
    //                 // println!(
    //                 //     "Adding: {}",
    //                 //     partial_checksum(right / 2, gap_index.0, needed)
    //                 // );
    //                 // println!(
    //                 //     "Removing: {}",
    //                 //     partial_checksum(right / 2, sim_indices[right],
    //                 // needed) );
    //
    //                 let rem = size - needed;
    //                 let rem_index = gap_index.0 + needed;
    //                 heaps[rem].push(Reverse(rem_index));
    //
    //                 // Stop searching
    //                 break 'outer;
    //             }
    //             _ => (),
    //         }
    //     }
    //
    //     right -= 2;
    // }

    let mut sum = 0;

    for (index, &size) in input.iter().enumerate().rev() {
        idx -= size;

        if index % 2 == 1 {
            continue;
        }

        let mut new_pos = idx;
        let mut gap_size = usize::MAX;

        for heap_size in size..10 {
            match heaps[heap_size].peek() {
                Some(gap_index) if gap_index.0 < new_pos => {
                    let Some(Reverse(gap_index)) = heaps[heap_size].pop()
                    else {
                        unreachable!()
                    };

                    if gap_index < new_pos {
                        new_pos = gap_index;
                        gap_size = heap_size;
                    }
                }
                _ => (),
            }
        }

        println!("Move {size} elements from {idx} to {new_pos}");

        if gap_size != usize::MAX {
            let remaining_gap = gap_size - size;
            heaps[remaining_gap].push(Reverse(new_pos + size));

            println!(
                "Creating new gap of size {remaining_gap} at {}",
                new_pos + size
            );
        }

        sum += partial_checksum(index / 2, new_pos, size);
    }

    // println!("Heaps: {heaps:?}");

    // todo!()
    sum
}

// For my input, the correct answer is:
// Part 1: 6201130364722
// Part 2:
