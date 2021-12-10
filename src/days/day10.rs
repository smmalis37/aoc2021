use crate::solver::Solver;

pub struct Day10;

type Num = u64;

impl<'a> Solver<'a> for Day10 {
    type Parsed = &'a [u8];
    type Output = Num;

    fn parse(input: &'a str) -> Self::Parsed {
        input.as_bytes()
    }

    fn part1(data: Self::Parsed) -> Self::Output {
        solve(&data, std::vec::Vec::clear)
    }

    fn part2(data: Self::Parsed) -> Self::Output {
        let mut scores = Vec::with_capacity(data.len() / 90);

        solve(
            &data,
            #[inline]
            |stack| {
                let mut score = 0;

                while let Some(o) = stack.pop() {
                    score = score * 5 + get_part2_score(o);
                }

                scores.push(score);
            },
        );

        let mid_pos = scores.len() / 2;
        *scores.select_nth_unstable(mid_pos).1
    }
}

#[inline]
fn solve(data: &<Day10 as Solver>::Parsed, mut newline: impl FnMut(&mut Vec<u8>)) -> Num {
    let mut pos = 0;
    let mut stack = Vec::with_capacity(110);
    let mut score = 0;

    while pos < data.len() {
        let c = data[pos];
        match c {
            b'(' | b'[' | b'{' | b'<' => stack.push(c),
            b')' | b']' | b'}' | b'>' => {
                if stack.pop().unwrap() != get_opener(c) {
                    score += get_part1_score(c);
                    stack.clear();
                    pos += memchr::memchr(b'\n', &data[pos..]).unwrap();
                }
            }
            b'\n' => newline(&mut stack),
            _ => unreachable!(),
        }
        pos += 1;
    }

    score
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
