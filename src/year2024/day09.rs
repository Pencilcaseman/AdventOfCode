#![warn(clippy::pedantic, clippy::nursery)]

type Input = Vec<usize>;

#[must_use]
pub fn parse(input: &str) -> Input {
    input.bytes().map(|b| (b - b'0') as usize).collect()
}

// #[must_use]
// pub fn part1(input: &Input) -> usize {
//     let mut real_index = 0;
//     let mut sim_index = 0;
//     let mut end_index = input.len() - 1;
//     let mut is_value = true;
//     let mut sum = 0;
//
//     let mut string = input.clone();
//
//     loop {
//         if string[real_index] == 0 {
//             end_index -= 2;
//             continue;
//         }
//
//         if is_value {
//             sum += sim_index * (real_index / 2);
//             string[real_index] -= 1;
//         } else {
//             sum += sim_index * (end_index / 2);
//             string[real_index] -= 1;
//             string[end_index] -= 1;
//         }
//
//         while string[real_index] == 0 {
//             real_index += 1;
//             is_value = !is_value;
//
//             if real_index == input.len() {
//                 break;
//             }
//         }
//
//         while string[end_index] == 0 {
//             // Skip the gap
//             end_index -= 2;
//
//             if end_index == 0 {
//                 break;
//             }
//         }
//
//         if string[real_index] == 0 || string[end_index] == 0 {
//             break;
//         }
//
//         sim_index += 1;
//     }
//
//     sum
// }

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
/// ```text
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
/// sum = x(s * n + LOOKUP[n])
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
    0
}

// For my input, the correct answer is:
// Part 1: 6201130364722
// Part 2:
