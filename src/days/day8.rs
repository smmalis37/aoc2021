use crate::{solver::Solver, util::*};

pub struct Day8;

impl<'a> Solver<'a> for Day8 {
    type Parsed = &'a [u8]; //Vec<([&'a [u8]; 10], [&'a [u8]; 4])>;
    type Output = u32;

    fn parse(input: &'a str) -> Self::Parsed {
        input.as_bytes()
    }

    fn part1(data: Self::Parsed) -> Self::Output {
        const PATTERN_LENGTH: usize = 61;
        let mut pos = 0;
        let mut res = 0;

        while pos < data.len() {
            pos += PATTERN_LENGTH;
            for _ in 0..4 {
                let space = memchr::memchr2(b' ', b'\n', &data[pos..]).unwrap();
                if [2, 3, 4, 7].contains(&space) {
                    res += 1;
                }
                pos += space + 1;
            }
        }

        res
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
