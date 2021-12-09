use crate::{solver::Solver, util::*};

pub struct Day9;

impl<'a> Solver<'a> for Day9 {
    type Parsed = (Vec<u8>, usize, usize);
    type Output = u32;

    fn parse(input: &'a str) -> Self::Parsed {
        let input = input.as_bytes();
        let line_len = memchr::memchr(b'\n', input).unwrap();
        let line_count = input.len() / line_len;
        let mut output = Vec::with_capacity(line_len * line_count);

        for x in input {
            match x {
                b'0'..=b'9' => output.push(x - b'0'),
                b'\n' => {}
                _ => unreachable!(),
            }
        }

        (output, line_len, line_count)
    }

    fn part1((data, line_len, _): Self::Parsed) -> Self::Output {
        let mut sum = 0;
        let line_len = 0isize.wrapping_sub_unsigned(line_len);

        for (i, val) in data.iter().enumerate() {
            let check = |a| {
                i.checked_add_signed(a)
                    .and_then(|x| data.get(x))
                    .map_or(true, |x| val < x)
            };
            if check(-1) & check(1) & check(-line_len) & check(line_len) {
                sum += (val + 1) as Self::Output;
            }
        }

        sum
    }

    fn part2(data: Self::Parsed) -> Self::Output {
        todo!()
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
        assert_eq!(Day9::part2(Day9::parse("")), 0);
    }
}
