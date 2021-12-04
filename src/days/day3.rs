use crate::{solver::Solver, util::*};

pub struct Day3;

impl<'a> Solver<'a> for Day3 {
    type Parsed = (usize, impl Iterator<Item = &'a [u8]> + Clone);
    type Output = u32;

    fn parse(input: &'a str) -> Self::Parsed {
        let len = memchr::memchr(b'\n', input.as_bytes()).unwrap();
        (
            len,
            input.as_bytes().chunks(len + 1).map(move |l| &l[..len]),
        )
    }

    fn part1((len, data): Self::Parsed) -> Self::Output {
        let mut counts = vec![0; len];
        let mut line_count = 0;

        for l in data {
            line_count += 1;
            for (digit, count) in l.iter().zip(counts.iter_mut()) {
                *count += (digit - b'0') as u16;
            }
        }

        let half_count = line_count / 2;
        let mut gamma = 0;
        let mut epsilon = 0;

        for c in counts {
            gamma <<= 1;
            epsilon <<= 1;

            let res = c as usize > half_count;

            gamma += res as u32;
            epsilon += !res as u32;
        }

        gamma * epsilon
    }

    fn part2((len, data): Self::Parsed) -> Self::Output {
        let mut data: Vec<_> = data.collect();
        data.sort_unstable();

        let mut res = 1;

        for (x, y1, y2, z2) in [(b'1', b'0', 0, 1), (b'0', b'1', 1, 0)] {
            let mut ratings = &data[..];
            let mut rating = 0;
            for i in 0..len {
                let mut pos = ratings.len() / 2;
                let bit = ratings[pos][i];
                let incr = match bit {
                    b'0' => 1,
                    b'1' => -1,
                    _ => unreachable!(),
                };

                rating <<= 1;
                rating += if ratings.len() == 1 {
                    (bit - b'0') as u32
                } else {
                    (bit == x) as u32
                };

                while ratings[pos][i] == bit && pos > 0 && pos < ratings.len() {
                    pos = pos.wrapping_add_signed(incr);
                }

                ratings = if bit == y1 {
                    &ratings[..pos + y2]
                } else {
                    &ratings[pos + z2..]
                };
            }

            res *= rating;
        }

        res
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
01010"
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
01010"
            )),
            230
        );
    }
}
