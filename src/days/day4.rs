use crate::solver::Solver;

pub struct Day4;

const ROW_SIZE: usize = 5;
const BOARD_SIZE: usize = ROW_SIZE * ROW_SIZE;
pub type Board = [u8; BOARD_SIZE];
pub type Index = Vec<[Option<u8>; 100]>;
type Num = u16;

impl<'a> Solver<'a> for Day4 {
    type Parsed = (Vec<u8>, Vec<Board>, Index);
    type Output = Num;

    fn parse(input: &'a str) -> Self::Parsed {
        let input = input.as_bytes();
        let line_length = memchr::memchr(b'\n', input).unwrap();
        let mut calls = Vec::with_capacity(line_length / 2);

        let mut num = 0;
        for c in &input[..=line_length] {
            match c {
                b'0'..=b'9' => {
                    num = (num * 10) + (c - b'0');
                }
                b',' | b'\n' => {
                    calls.push(num);
                    num = 0;
                }
                _ => {
                    unreachable!();
                }
            }
        }

        let board_input = &input[line_length + 1..];
        let board_char_count = BOARD_SIZE * 3 + 1;
        let board_count = board_input.len() / board_char_count;
        let mut boards = vec![Board::default(); board_count];
        let mut index: Index = vec![[None; 100]; board_count];

        for (i, b) in board_input.chunks_exact(board_char_count).enumerate() {
            let mut pos = 0;
            for c in b[1..].chunks_exact(3) {
                match c {
                    [a @ b'0'..=b'9', b @ b'0'..=b'9', b' ' | b'\n'] => {
                        let num = (a - b'0') * 10 + (b - b'0');
                        boards[i][pos as usize] = num;
                        index[i][num as usize] = Some(pos);
                        pos += 1;
                    }
                    [b' ', b @ b'0'..=b'9', b' ' | b'\n'] => {
                        let num = b - b'0';
                        boards[i][pos as usize] = num;
                        index[i][num as usize] = Some(pos);
                        pos += 1;
                    }
                    _ => {
                        unreachable!();
                    }
                }
            }
        }

        (calls, boards, index)
    }

    fn part1(data: Self::Parsed) -> Self::Output {
        solve(data, || true)
    }

    fn part2(data: Self::Parsed) -> Self::Output {
        let mut count = 0;
        let done = data.1.len();
        solve(data, || {
            count += 1;
            count == done
        })
    }
}

#[inline]
fn solve(
    (calls, boards, index): <Day4 as Solver>::Parsed,
    mut break_time: impl FnMut() -> bool,
) -> Num {
    let mut called = vec![[false; BOARD_SIZE]; boards.len()];
    let mut won = vec![false; boards.len()];

    let (winner, last_call) = 'w: {
        for c in calls {
            for (i, pos) in index
                .iter()
                .enumerate()
                .filter_map(|(i, r)| r[c as usize].map(|x| (i, x)))
            {
                if won[i] {
                    continue;
                }

                let pos = pos as usize;
                debug_assert!(boards[i][pos] == c);
                called[i][pos] = true;

                let row = pos / ROW_SIZE;
                let col = pos % ROW_SIZE;
                if called[i].chunks_exact(ROW_SIZE).all(|r| r[col])
                    || called[i][row * ROW_SIZE..(row + 1) * ROW_SIZE]
                        .iter()
                        .all(|x| *x)
                {
                    won[i] = true;

                    if break_time() {
                        break 'w (i, c);
                    }
                }
            }
        }
        unreachable!()
    };

    boards[winner]
        .into_iter()
        .zip(called[winner].into_iter())
        .filter(|(_, called)| !called)
        .map(|(num, _)| num as Num)
        .sum::<Num>()
        * (last_call as Num)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d4p1() {
        assert_eq!(
            Day4::part1(Day4::parse(
                "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7
"
            )),
            4512
        );
    }

    #[test]
    fn d4p2() {
        assert_eq!(
            Day4::part2(Day4::parse(
                "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7
"
            )),
            1924
        );
    }
}
