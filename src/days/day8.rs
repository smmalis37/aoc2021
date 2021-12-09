use crate::{solver::Solver, util::*};

pub struct Day8;

impl<'a> Solver<'a> for Day8 {
    type Parsed = Vec<([&'a [u8]; 10], [&'a [u8]; 4])>;
    type Output = usize;

    fn parse(input: &'a str) -> Self::Parsed {
        let input = input.as_bytes();
        let mut iter = memchr::memchr2_iter(b' ', b'\n', input).peekable();
        let mut res = Vec::with_capacity(input.len() / 72);
        let mut pos = 0;

        while iter.peek().is_some() {
            let digits = [
                &input[pos..*iter.peek().unwrap()],
                &input[iter.next().unwrap() + 1..*iter.peek().unwrap()],
                &input[iter.next().unwrap() + 1..*iter.peek().unwrap()],
                &input[iter.next().unwrap() + 1..*iter.peek().unwrap()],
                &input[iter.next().unwrap() + 1..*iter.peek().unwrap()],
                &input[iter.next().unwrap() + 1..*iter.peek().unwrap()],
                &input[iter.next().unwrap() + 1..*iter.peek().unwrap()],
                &input[iter.next().unwrap() + 1..*iter.peek().unwrap()],
                &input[iter.next().unwrap() + 1..*iter.peek().unwrap()],
                &input[iter.next().unwrap() + 1..*iter.peek().unwrap()],
            ];
            let separator = iter.next();
            debug_assert!(input[separator.unwrap() + 1] == b'|');
            let outputs = [
                &input[iter.next().unwrap() + 1..*iter.peek().unwrap()],
                &input[iter.next().unwrap() + 1..*iter.peek().unwrap()],
                &input[iter.next().unwrap() + 1..*iter.peek().unwrap()],
                &input[iter.next().unwrap() + 1..*iter.peek().unwrap()],
            ];
            res.push((digits, outputs));
            pos = iter.next().unwrap();
        }

        res
    }

    fn part1(data: Self::Parsed) -> Self::Output {
        data.into_iter()
            .flat_map(|(_, outputs)| outputs)
            .filter(|o| [2, 3, 4, 7].contains(&o.len()))
            .count()
    }

    fn part2(data: Self::Parsed) -> Self::Output {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d8p1() {
        assert_eq!(
            Day8::part1(Day8::parse(
                "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb |
fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec |
fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef |
cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega |
efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga |
gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf |
gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf |
cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd |
ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg |
gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc |
fgae cfgab fg bagce
"
            )),
            26
        );
    }

    #[test]
    fn d8p2() {
        assert_eq!(Day8::part2(Day8::parse("")), 0);
    }
}
