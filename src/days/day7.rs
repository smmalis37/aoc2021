use crate::solver::Solver;

pub struct Day7;

impl<'a> Solver<'a> for Day7 {
    type Parsed = Vec<u16>;
    type Output = u32;

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
        left.iter()
            .map(|x| (val - x) as Self::Output)
            .sum::<Self::Output>()
            + right
                .iter()
                .map(|x| (x - val) as Self::Output)
                .sum::<Self::Output>()
    }

    fn part2(data: Self::Parsed) -> Self::Output {
        #[allow(clippy::cast_possible_truncation)]
        let mean = data
            .iter()
            .map(|&x| x as Self::Output)
            .sum::<Self::Output>()
            .unstable_div_floor(data.len() as Self::Output);

        data.iter()
            .flat_map(|&x| 1..=(mean.abs_diff(x as Self::Output)))
            .sum()
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
