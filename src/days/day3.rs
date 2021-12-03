use crate::{solver::Solver, util::*};

pub struct Day3;

impl<'a> Solver<'a> for Day3 {
    type Parsed = Vec<&'a [u8]>;
    type Output = u32;

    fn parse(input: &'a str) -> Self::Parsed {
        input.as_bytes().split(bytelines).collect()
    }

    fn part1(data: Self::Parsed) -> Self::Output {
        let half_count = data.len() / 2;
        let mut gamma = 0;
        let mut epsilon = 0;

        for i in 0..data[0].len() {
            gamma <<= 1;
            epsilon <<= 1;

            if data.iter().map(|l| (l[i] - b'0') as u16).sum::<u16>() as usize > half_count {
                gamma += 1;
            } else {
                epsilon += 1;
            }
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
