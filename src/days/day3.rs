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

        let mut o2ratings = &data[..];
        let mut o2rating = 0;
        let mut co2ratings = &data[..];
        let mut co2rating = 0;

        for i in 0..len {
            let mut pos = o2ratings.len() / 2;
            let bit = o2ratings[pos][i];
            let incr = match bit {
                b'0' => 1,
                b'1' => -1,
                _ => unreachable!(),
            };

            o2rating <<= 1;
            o2rating += (bit == b'1') as u32;

            while o2ratings[pos][i] == bit {
                pos = pos.wrapping_add_signed(incr);
            }

            o2ratings = match bit {
                b'0' => &o2ratings[..pos],
                b'1' => &o2ratings[pos + 1..],
                _ => unreachable!(),
            }
        }

        for i in 0..len {
            println!("{:?}", co2ratings);
            let mut pos = co2ratings.len() / 2;
            let bit = co2ratings[pos][i];
            let incr = match bit {
                b'0' => 1,
                b'1' => -1,
                _ => unreachable!(),
            };

            co2rating <<= 1;
            co2rating += if co2ratings.len() == 1 {
                (bit - b'0') as u32
            } else {
                (bit == b'0') as u32
            };
            println!("{:b}", co2rating);

            while co2ratings[pos][i] == bit && pos > 0 && pos < co2ratings.len() {
                pos = pos.wrapping_add_signed(incr);
            }

            co2ratings = match bit {
                b'0' => &co2ratings[pos..],
                b'1' => &co2ratings[..=pos],
                _ => unreachable!(),
            }
        }

        println!("{} {}", o2rating, co2rating);

        o2rating * co2rating
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
