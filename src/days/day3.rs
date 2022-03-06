use crate::{solver::Solver, util::*};

pub struct Day3;

impl<'a> Solver<'a> for Day3 {
    type Parsed = (usize, impl Iterator<Item = &'a [u8]> + Clone);
    type Output = usize;

    fn parse(input: &'a str) -> Self::Parsed {
        let len = memchr::memchr(b'\n', input.as_bytes()).unwrap();
        (
            len,
            input
                .as_bytes()
                .chunks_exact(len + 1)
                .map(move |l| &l[..len]),
        )
    }

    fn part1((len, data): Self::Parsed) -> Self::Output {
        let mut counts = vec![0; len];
        let mut line_count = 0;

        for l in data {
            line_count += 1;
            for (digit, count) in l.iter().zip(counts.iter_mut()) {
                *count += u16::from(digit - b'0');
            }
        }

        let half_count = line_count / 2;
        let mut gamma = 0;
        let mut epsilon = 0;

        for c in counts {
            gamma <<= 1;
            epsilon <<= 1;

            let res = c > half_count;

            gamma += Self::Output::from(res);
            epsilon += Self::Output::from(!res);
        }

        gamma * epsilon
    }

    fn part2((len, data): Self::Parsed) -> Self::Output {
        let mut line_count = 0;
        let mut counts = vec![0_u16; 1 << len];

        for l in data {
            line_count += 1;
            counts[l.parse_binary::<usize>()] += 1;
        }

        let mut offset_most = 0;
        let mut offset_least = 0;
        let mut total_most = line_count;
        let mut total_least = line_count;
        let mut size = 1 << len;

        for _ in 0..len {
            size /= 2;
            let (mut most0, mut least0) = (0, 0);

            for j in 0..size {
                most0 += counts[offset_most + j];
                least0 += counts[offset_least + j];
            }

            let most1 = total_most - most0;
            let least1 = total_least - least0;

            total_most = if most1 >= most0 {
                offset_most += size;
                most1
            } else {
                most0
            };

            total_least = if least0 != 0 && (least0 <= least1 || least1 == 0) {
                least0
            } else {
                offset_least += size;
                least1
            };
        }

        debug_assert!(counts[offset_most] != 0);
        debug_assert!(counts[offset_least] != 0);

        offset_most * offset_least
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d3p1() {
        assert_eq!(
            Day3::part1(Day3::parse(
                "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010
"
            )),
            198
        );
    }

    #[test]
    fn d3p2() {
        assert_eq!(
            Day3::part2(Day3::parse(
                "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010
"
            )),
            230
        );
    }
}
