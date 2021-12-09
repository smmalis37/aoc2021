use arrayvec::ArrayVec;

use crate::solver::Solver;

pub struct Day8;

type N = u8;

impl<'a> Solver<'a> for Day8 {
    type Parsed = Vec<([N; 10], [N; 4])>;
    type Output = usize;

    fn parse(input: &'a str) -> Self::Parsed {
        let input = input.as_bytes();
        let mut iter = memchr::memchr2_iter(b' ', b'\n', input);
        let mut res = Vec::with_capacity(input.len() / 72);
        let mut pos = 0;

        while pos < input.len() {
            let mut digits = ArrayVec::new();
            inner_parse(input, &mut digits, &mut iter, &mut pos);

            debug_assert!(input[pos] == b'|');
            pos = iter.next().unwrap() + 1;

            let mut outputs = ArrayVec::new();
            inner_parse(input, &mut outputs, &mut iter, &mut pos);

            res.push((digits.into_inner().unwrap(), outputs.into_inner().unwrap()));
        }

        res
    }

    fn part1(data: Self::Parsed) -> Self::Output {
        data.into_iter()
            .flat_map(|(_, outputs)| outputs)
            .filter(|o| [2, 3, 4, 7].contains(&o.count_ones()))
            .count() as Self::Output
    }

    fn part2(data: Self::Parsed) -> Self::Output {
        let mut sum = 0;
        for (digits, output) in data {
            let [mut d0, mut d1, mut d2, mut d3, mut d4, mut d5, mut d6, mut d7, mut d8, mut d9] =
                [0u8; 10];

            for (d, i) in digits.iter().zip(0..) {
                match d.count_ones() {
                    2 => d1 = i,
                    4 => d4 = i,
                    3 => d7 = i,
                    7 => d8 = i,
                    _ => {}
                }
            }

            for (d, i) in digits.iter().zip(0..) {
                match (d ^ digits[d1 as usize]).count_ones() {
                    3 => d3 = i,
                    6 => d6 = i,
                    _ => {}
                }

                match (d ^ digits[d4 as usize]).count_ones() {
                    5 => d2 = i,
                    2 if i != d1 => d9 = i,
                    _ => {}
                }
            }

            for (d, i) in digits.iter().zip(0..) {
                match (d ^ digits[d6 as usize]).count_ones() {
                    1 if i != d8 => d5 = i,
                    2 if i != d9 => d0 = i,
                    _ => {}
                }
            }

            let map = [d0, d1, d2, d3, d4, d5, d6, d7, d8, d9];
            let mut res = 0;
            for &o in &output {
                let digit = map.iter().position(|&x| digits[x as usize] == o).unwrap();
                res = (res * 10) + digit;
            }
            sum += res;
        }

        sum
    }
}

#[inline]
fn inner_parse<const S: usize>(
    inp: &[u8],
    out: &mut ArrayVec<N, S>,
    i: &mut impl Iterator<Item = usize>,
    p: &mut usize,
) {
    for _ in 0..S {
        let space = i.next().unwrap();
        let word = &inp[*p..space];
        assert!(word.len() >= 2);
        let mut x = 0;
        for c in word {
            x |= 1 << (c - b'a');
        }
        out.push(x);
        *p = space + 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d8p1() {
        assert_eq!(
            Day8::part1(Day8::parse(
                "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab |
cdfeb fcadb cdfeb cdbaf
"
            )),
            0
        );

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
        assert_eq!(
            Day8::part2(Day8::parse(
                "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab |
cdfeb fcadb cdfeb cdbaf
"
            )),
            5353
        );

        assert_eq!(
            Day8::part2(Day8::parse(
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
            61229
        );
    }
}
