use crate::solver::Solver;

pub struct Day2;

#[derive(Clone, Copy)]
pub enum Direction {
    Forward,
    Down,
    Up,
}

impl<'a> Solver<'a> for Day2 {
    type Parsed = impl Iterator<Item = (Direction, u8)> + Clone;
    type Output = u32;

    fn parse(input: &'a str) -> Self::Parsed {
        let mut input = input.as_bytes().iter();
        std::iter::from_fn(move || {
            input.next().map(|d| {
                let (dir, pos) = match d {
                    b'f' => (Direction::Forward, "forward".len()),
                    b'd' => (Direction::Down, "down".len()),
                    b'u' => (Direction::Up, "up".len()),
                    _ => unreachable!(),
                };

                let amount = input.nth(pos).unwrap() - b'0';
                input.next();

                (dir, amount)
            })
        })
    }

    fn part1(data: Self::Parsed) -> Self::Output {
        let mut horiz = 0;
        let mut depth = 0;

        for (d, a) in data {
            let a = a as Self::Output;
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

        for (d, a) in data {
            let a = a as Self::Output;
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
forward 2
"
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
forward 2
"
            )),
            900
        );
    }
}
