use crate::{solver::Solver, util::*};

pub struct Day2;

type Num = u32;

#[derive(Clone, Copy)]
pub enum Direction {
    Forward,
    Down,
    Up,
}

#[derive(Clone, Copy)]
pub struct Move {
    dir: Direction,
    amount: Num,
}

impl<'a> Solver<'a> for Day2 {
    type Parsed = Vec<Move>;
    type Output = Num;

    fn parse(input: &'a str) -> Self::Parsed {
        input
            .as_bytes()
            .split(bytelines)
            .map(|l| l.split(|&x| x == b' '))
            .map(|mut l| Move {
                dir: match l.next().unwrap()[0] {
                    b'f' => Direction::Forward,
                    b'd' => Direction::Down,
                    b'u' => Direction::Up,
                    _ => unreachable!(),
                },
                amount: l.next().unwrap().parse().unwrap(),
            })
            .collect()
    }

    fn part1(data: Self::Parsed) -> Self::Output {
        let mut horiz = 0;
        let mut depth = 0;

        for m in data {
            match m.dir {
                Direction::Forward => horiz += m.amount,
                Direction::Down => depth += m.amount,
                Direction::Up => depth -= m.amount,
            }
        }

        horiz * depth
    }

    fn part2(data: Self::Parsed) -> Self::Output {
        let mut horiz = 0;
        let mut depth = 0;
        let mut aim = 0;

        for m in data {
            match m.dir {
                Direction::Forward => {
                    horiz += m.amount;
                    depth += aim * m.amount;
                }
                Direction::Down => aim += m.amount,
                Direction::Up => aim -= m.amount,
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
