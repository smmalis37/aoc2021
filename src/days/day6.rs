use crate::solver::Solver;

pub struct Day6;
type Num = u64;

impl<'a> Solver<'a> for Day6 {
    type Parsed = [Num; 7];
    type Output = Num;

    fn parse(input: &'a str) -> Self::Parsed {
        let mut result = [0; 7];

        for n in input.as_bytes().iter().step_by(2) {
            result[usize::from(n - b'0')] += 1;
        }

        result
    }

    fn part1(data: Self::Parsed) -> Self::Output {
        solve(data, 80)
    }

    fn part2(data: Self::Parsed) -> Self::Output {
        solve(data, 256)
    }
}

fn solve(mut data: <Day6 as Solver>::Parsed, count: usize) -> Num {
    let mut newbies = [0; 2];

    for _ in 0..count {
        let no_longer_new = newbies[0];
        newbies[0] = newbies[1];
        newbies[1] = data[0];
        data.rotate_left(1);
        data[6] += no_longer_new;
    }

    data.into_iter().sum::<Num>() + newbies.into_iter().sum::<Num>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d6p1() {
        assert_eq!(Day6::part1(Day6::parse("3,4,3,1,2 ")), 5934);
    }

    #[test]
    fn d6p2() {
        assert_eq!(Day6::part2(Day6::parse("3,4,3,1,2 ")), 26984457539);
    }
}
