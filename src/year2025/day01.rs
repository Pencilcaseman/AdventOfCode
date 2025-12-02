use num_traits::Euclid;

type Input = Vec<i32>;

pub fn parse(input: &str) -> Input {
    input
        .lines()
        .map(|substr| {
            let mut bytes = substr.bytes();

            let sign: i32 = match bytes.next() {
                Some(b'L') => -1,
                Some(b'R') => 1,
                _ => unreachable!(),
            };

            let num =
                bytes.fold(0, |sum, byte| sum * 10 + (byte - b'0') as i32);

            num * sign
        })
        .collect()
}

pub fn part1(input: &Input) -> i32 {
    input
        .iter()
        .fold((0, 50), |(mut count, mut angle), delta| {
            angle = (angle + delta) % 100;
            if angle == 0 {
                count += 1;
            }

            (count, angle)
        })
        .0
}

pub fn part2(input: &Input) -> i32 {
    input
        .iter()
        .fold((50, 0), |(mut angle, mut count), &delta| {
            if delta > 0 {
                let (full, rem) = delta.div_rem_euclid(&100);
                count += full;

                angle += rem;

                if angle > 99 {
                    angle -= 100;
                    count += 1;
                }
            } else if delta < 0 {
                let (full, rem) = (-delta).div_rem_euclid(&100);
                count += full;

                let mut new_angle = angle - rem;

                if new_angle < 0 {
                    new_angle += 100;

                    if angle > 0 {
                        count += 1;
                    }
                } else if new_angle == 0 {
                    count += 1;
                }

                angle = new_angle;
            }

            (angle, count)
        })
        .1
}

// 6430 too high
