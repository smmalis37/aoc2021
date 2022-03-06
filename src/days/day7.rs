use crate::solver::Solver;

pub struct Day7;

type Num = u32;

impl<'a> Solver<'a> for Day7 {
    type Parsed = Vec<Num>;
    type Output = Num;

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

    fn part1(mut data: Self::Parsed) -> Self::Output {
        let med_pos = data.len() / 2;
        let (left, &mut val, right) = data.select_nth_unstable(med_pos);

        left.iter().map(|x| val - x).sum::<Num>() + right.iter().map(|x| x - val).sum::<Num>()
    }

    #[allow(clippy::as_conversions, clippy::cast_possible_truncation)] // u32 and usize
    fn part2(data: Self::Parsed) -> Self::Output {
        let mean = data.iter().sum::<Num>().div_floor(data.len() as Num);

        data.into_iter().flat_map(|x| 1..=mean.abs_diff(x)).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d7p1() {
        assert_eq!(Day7::part1(Day7::parse("16,1,2,0,4,2,7,1,2,14 ")), 37);
    }

    #[test]
    fn d7p2() {
        assert_eq!(Day7::part2(Day7::parse("16,1,2,0,4,2,7,1,2,14 ")), 170);
    }
}
