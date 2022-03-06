use std::cmp::{max, min};

use either::Either;

use crate::solver::Solver;

pub struct Day5;

impl<'a> Solver<'a> for Day5 {
    type Parsed = Vec<(u16, u16, u16, u16)>;
    type Output = usize;

    fn parse(input: &'a str) -> Self::Parsed {
        let mut input = input.as_bytes();
        let mut result = Vec::with_capacity(input.len() / "##,## -> ##,##".len());

        #[allow(clippy::shadow_unrelated)] // Clippy bug?
        while !input.is_empty() {
            let (x1, x2, y1, y2, mut n);
            (x1, n) = lexical_core::parse_partial(input).unwrap();
            input = &input[n + 1..];
            (y1, n) = lexical_core::parse_partial(input).unwrap();
            input = &input[n + 4..];
            (x2, n) = lexical_core::parse_partial(input).unwrap();
            input = &input[n + 1..];
            (y2, n) = lexical_core::parse_partial(input).unwrap();
            input = &input[n + 1..];

            result.push((x1, y1, x2, y2));
        }

        result
    }

    fn part1(data: Self::Parsed) -> Self::Output {
        let mut grid = vec![[0_u16; 1000]; 1000];

        for (x1, y1, x2, y2) in data {
            if x1 == x2 {
                for y in min(y1, y2)..=max(y1, y2) {
                    grid[usize::from(x1)][usize::from(y)] += 1;
                }
            } else if y1 == y2 {
                for x in min(x1, x2)..=max(x1, x2) {
                    grid[usize::from(x)][usize::from(y1)] += 1;
                }
            }
        }

        grid.into_iter().flatten().filter(|&x| x > 1).count()
    }

    fn part2(data: Self::Parsed) -> Self::Output {
        let mut grid = vec![[0_u16; 1000]; 1000];

        for (x1, y1, x2, y2) in data {
            if x1 == x2 {
                for y in min(y1, y2)..=max(y1, y2) {
                    grid[usize::from(y)][usize::from(x1)] += 1;
                }
            } else if y1 == y2 {
                for x in min(x1, x2)..=max(x1, x2) {
                    grid[usize::from(y1)][usize::from(x)] += 1;
                }
            } else {
                let x_range = if x1 < x2 {
                    Either::Left(x1..=x2)
                } else {
                    Either::Right((x2..=x1).rev())
                };

                let y_range = if y1 < y2 {
                    Either::Left(y1..=y2)
                } else {
                    Either::Right((y2..=y1).rev())
                };

                for (x, y) in x_range.zip(y_range) {
                    grid[usize::from(y)][usize::from(x)] += 1;
                }
            }
        }

        grid.into_iter().flatten().filter(|&x| x > 1).count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d5p1() {
        assert_eq!(
            Day5::part1(Day5::parse(
                "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2
"
            )),
            5
        );
    }

    #[test]
    fn d5p2() {
        assert_eq!(
            Day5::part2(Day5::parse(
                "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2
"
            )),
            12
        );
    }
}
