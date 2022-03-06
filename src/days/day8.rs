use arrayvec::ArrayVec;

use crate::solver::Solver;

pub struct Day8;

type N = u8;

impl<'a> Solver<'a> for Day8 {
    type Parsed = Vec<([N; 10], [N; 4])>;
    type Output = usize;

    fn parse(input: &'a str) -> Self::Parsed {
        let mut input = input.as_bytes();
        let mut res = Vec::with_capacity(input.len() / 72);

        while !input.is_empty() {
            let mut digits = ArrayVec::new();
            inner_parse(&mut input, &mut digits);

            debug_assert!(input[0] == b'|');
            input = &input[2..];

            let mut outputs = ArrayVec::new();
            inner_parse(&mut input, &mut outputs);

            res.push((digits.into_inner().unwrap(), outputs.into_inner().unwrap()));
        }

        res
    }

    fn part1(data: Self::Parsed) -> Self::Output {
        data.into_iter()
            .flat_map(|(_, outputs)| outputs)
            .filter(|o| [2, 3, 4, 7].contains(&o.count_ones()))
            .count()
    }

    fn part2(data: Self::Parsed) -> Self::Output {
        let mut sum = 0;
        for (digits, output) in data {
            let mut map = [0_u16; 10];

            for (d, i) in digits.iter().zip(0..) {
                match d.count_ones() {
                    2 => map[1] = i,
                    4 => map[4] = i,
                    3 => map[7] = i,
                    7 => map[8] = i,
                    _ => {}
                }
            }

            for (d, i) in digits.iter().zip(0..) {
                match (d ^ digits[usize::from(map[1])]).count_ones() {
                    3 => map[3] = i,
                    6 => map[6] = i,
                    _ => {}
                }

                match (d ^ digits[usize::from(map[4])]).count_ones() {
                    5 => map[2] = i,
                    2 if i != map[1] => map[9] = i,
                    _ => {}
                }
            }

            for (d, i) in digits.iter().zip(0..) {
                match (d ^ digits[usize::from(map[6])]).count_ones() {
                    1 if i != map[8] => map[5] = i,
                    2 if i != map[9] => map[0] = i,
                    _ => {}
                }
            }

            let mut res = 0;
            for &o in &output {
                let digit = map
                    .iter()
                    .position(|&x| digits[usize::from(x)] == o)
                    .unwrap();
                res = (res * 10) + digit;
            }
            sum += res;
        }

        sum
    }
}

#[inline]
fn inner_parse<const S: usize>(input: &mut &[u8], out: &mut ArrayVec<N, S>) {
    for _ in 0..S {
        let space = memchr::memchr2(b' ', b'\n', input).unwrap();
        assert!(space >= 2);

        let mut x = 0;
        for c in &input[..space] {
            x |= 1 << (c - b'a');
        }
        out.push(x);

        *input = &input[space + 1..];
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
