use std::collections::BinaryHeap;

use crate::{solver::Solver, util::*};

pub struct Day9;

impl<'a> Solver<'a> for Day9 {
    type Parsed = Grid<u8>;
    type Output = u32;

    fn parse(input: &'a str) -> Self::Parsed {
        let input = input.as_bytes();
        let line_length = memchr::memchr(b'\n', input).unwrap();
        let line_count = input.len() / (line_length + 1);
        let mut output = Vec::with_capacity(line_length * line_count);

        for x in input {
            match x {
                b'0'..=b'9' => output.push(x - b'0'),
                b'\n' => {}
                _ => unreachable!(),
            }
        }

        Grid::from_vec(output, line_length, line_count)
    }

    fn part1(data: Self::Parsed) -> Self::Output {
        let mut sum = 0;

        for r in 0..data.line_count() {
            for c in 0..data.line_length() {
                let check = |ra, ca| {
                    r.checked_add_signed(ra)
                        .zip(c.checked_add_signed(ca))
                        .and_then(|(r2, c2)| data.get(r2).and_then(|r3| r3.get(c2)))
                        .map_or(true, |&x| data[r][c] < x)
                };
                if check(0, -1) && check(0, 1) && check(-1, 0) && check(1, 0) {
                    sum += (data[r][c] + 1) as Self::Output;
                }
            }
        }

        sum
    }

    fn part2(mut data: Self::Parsed) -> Self::Output {
        let mut stack = Vec::with_capacity(data.line_length());
        let mut basins = BinaryHeap::with_capacity(data.line_length());

        for r in 0..data.line_count() {
            for c in 0..data.line_length() {
                if data[r][c] != 9 {
                    stack.push((r, c));
                    let mut count = 0;

                    while let Some((sr, sc)) = stack.pop() {
                        let cell = data.get(sr).and_then(|x| x.get(sc));
                        if cell.is_some() && cell != Some(&9) {
                            data[sr][sc] = 9;
                            count += 1;
                            stack.extend_from_slice(&[
                                (sr, sc + 1),
                                (sr, sc.wrapping_add_signed(-1)),
                                (sr + 1, sc),
                                (sr.wrapping_add_signed(-1), sc),
                            ]);
                        }
                    }

                    basins.push(count);
                }
            }
        }

        basins.into_iter_sorted().take(3).product()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d9p1() {
        assert_eq!(
            Day9::part1(Day9::parse(
                "2199943210
3987894921
9856789892
8767896789
9899965678
"
            )),
            15
        );
    }

    #[test]
    fn d9p2() {
        assert_eq!(
            Day9::part2(Day9::parse(
                "2199943210
3987894921
9856789892
8767896789
9899965678
"
            )),
            1134
        );
    }
}
