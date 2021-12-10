use crate::{solver::Solver, util::*};

pub struct Day10;

type Num = u64;

impl<'a> Solver<'a> for Day10 {
    type Parsed = impl Iterator<Item = &'a [u8]> + Clone;
    type Output = Num;

    fn parse(input: &'a str) -> Self::Parsed {
        input.as_bytes().split(bytelines)
    }

    fn part1(data: Self::Parsed) -> Self::Output {
        let mut score = 0;
        'l: for l in data {
            let mut stack = Vec::new();
            for &c in l {
                match c {
                    b'(' | b'[' | b'{' | b'<' => stack.push(c),
                    b')' | b']' | b'}' | b'>' if stack.pop().unwrap() != get_opener(c) => {
                        score += get_part1_score(c);
                        continue 'l;
                    }
                    _ => {}
                }
            }
        }

        score
    }

    fn part2(data: Self::Parsed) -> Self::Output {
        let mut scores = Vec::new();
        'l: for l in data {
            let mut stack = Vec::new();
            for &c in l {
                match c {
                    b'(' | b'[' | b'{' | b'<' => stack.push(c),
                    b')' | b']' | b'}' | b'>' if stack.pop().unwrap() != get_opener(c) => {
                        continue 'l
                    }
                    _ => {}
                }
            }

            let mut score = 0;

            while let Some(o) = stack.pop() {
                score = score * 5 + get_part2_score(o);
            }

            scores.push(score);
        }

        let mid_pos = scores.len() / 2;
        *scores.select_nth_unstable(mid_pos).1
    }
}

#[inline]
const fn get_opener(c: u8) -> u8 {
    match c {
        b')' => b'(',
        b']' => b'[',
        b'}' => b'{',
        b'>' => b'<',
        _ => unreachable!(),
    }
}

#[inline]
const fn get_part1_score(c: u8) -> Num {
    match c {
        b')' => 3,
        b']' => 57,
        b'}' => 1197,
        b'>' => 25137,
        _ => unreachable!(),
    }
}

#[inline]
const fn get_part2_score(c: u8) -> Num {
    match c {
        b'(' => 1,
        b'[' => 2,
        b'{' => 3,
        b'<' => 4,
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d10p1() {
        assert_eq!(
            Day10::part1(Day10::parse(
                "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]
"
            )),
            26397
        );
    }

    #[test]
    fn d10p2() {
        assert_eq!(
            Day10::part2(Day10::parse(
                "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]
"
            )),
            288957
        );
    }
}
