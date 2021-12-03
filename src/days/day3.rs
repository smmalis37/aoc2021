use crate::{solver::Solver, util::*};

pub struct Day3;

impl<'a> Solver<'a> for Day3 {
    type Parsed = Vec<&'a [u8]>;
    type Output = u32;

    fn parse(input: &'a str) -> Self::Parsed {
        input.as_bytes().split(bytelines).collect()
    }

    fn part1(data: Self::Parsed) -> Self::Output {
        let len = data[0].len();
        assert!(len <= 16);
        let mut counts = vec![0; len];
        let half_count = data.len() / 2;

        for l in data {
            for (digit, count) in l.iter().zip(counts.iter_mut()) {
                *count += (digit - b'0') as u16;
            }
        }

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

    fn part2(data: Self::Parsed) -> Self::Output {
        // sort, go to midpoint, find range, repeat
        todo!()
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
        assert_eq!(Day3::part2(Day3::parse("")), 0);
    }
}
