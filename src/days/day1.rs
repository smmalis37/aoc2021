use crate::solver::Solver;

pub struct Day1;

type Num = u16;

impl<'a> Solver<'a> for Day1 {
    type Parsed = Vec<Num>;
    type Output = usize;

    fn parse(input: &'a str) -> Self::Parsed {
        let mut res = Vec::with_capacity(input.len() / 2);
        let mut input = input.as_bytes();

        while !input.is_empty() {
            let (num, size) = lexical_core::parse_partial(input).unwrap();
            input = &input[size + 1..];
            res.push(num);
        }

        res
    }

    fn part1(data: Self::Parsed) -> Self::Output {
        solve::<2>(&data)
    }

    fn part2(data: Self::Parsed) -> Self::Output {
        solve::<4>(&data)
    }
}

#[inline]
fn solve<'a, const N: usize>(data: &<Day1 as Solver>::Parsed) -> <Day1 as Solver<'a>>::Output {
    data.array_windows::<N>()
        .filter(|x| x[N - 1] > x[0])
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d1p1() {
        assert_eq!(
            Day1::part1(Day1::parse(
                "199
200
208
210
200
207
240
269
260
263
"
            )),
            7
        );
    }

    #[test]
    fn d1p2() {
        assert_eq!(
            Day1::part2(Day1::parse(
                "199
200
208
210
200
207
240
269
260
263
"
            )),
            5
        );
    }
}
