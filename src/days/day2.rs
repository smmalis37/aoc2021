use crate::{solver::Solver, util::*};

pub struct Day2;

type Num = u32;

#[derive(Clone, Copy)]
pub enum Direction {
    Forward,
    Down,
    Up,
}

impl<'a> Solver<'a> for Day2 {
    type Parsed = Vec<(Direction, Num)>;
    type Output = Num;

    fn parse(input: &'a str) -> Self::Parsed {
        input
            .as_bytes()
            .split(bytelines)
            .map(|l| match l[0] {
                b'f' => (Direction::Forward, &l[8..]),
                b'd' => (Direction::Down, &l[5..]),
                b'u' => (Direction::Up, &l[3..]),
                _ => unreachable!(),
            })
            .inspect(|(_, a)| debug_assert!(a.len() == 1 && (b'0'..=b'9').contains(&a[0])))
            .map(|(d, a)| (d, (a[0] - b'0') as Num))
            .collect()
    }

    fn part1(data: Self::Parsed) -> Self::Output {
        let mut horiz = 0;
        let mut depth = 0;

        for &(d, a) in &data {
            match d {
                Direction::Forward => horiz += a,
                Direction::Down => depth += a,
                Direction::Up => depth -= a,
            }
        }

        horiz * depth
    }

    fn part2(data: Self::Parsed) -> Self::Output {
        let mut horiz = 0;
        let mut depth = 0;
        let mut aim = 0;

        for &(d, a) in &data {
            match d {
                Direction::Forward => {
                    horiz += a;
                    depth += aim * a;
                }
                Direction::Down => aim += a,
                Direction::Up => aim -= a,
            }
        }

        horiz * depth
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d2p1() {
        assert_eq!(
            Day2::part1(Day2::parse(
                "forward 5
down 5
forward 8
up 3
down 8
forward 2"
            )),
            150
        );
    }

    #[test]
    fn d2p2() {
        assert_eq!(
            Day2::part2(Day2::parse(
                "forward 5
down 5
forward 8
up 3
down 8
forward 2"
            )),
            900
        );
    }
}
