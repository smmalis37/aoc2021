use crate::solver::Solver;

pub struct Day4;

pub type Board = [[u8; 5]; 5];

impl<'a> Solver<'a> for Day4 {
    type Parsed = (Vec<u8>, Vec<Board>);
    type Output = u16;

    fn parse(input: &'a str) -> Self::Parsed {
        let input = input.as_bytes();
        let line_length = memchr::memchr(b'\n', input).unwrap();
        let mut calls = Vec::with_capacity(line_length / "##,".len());

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

        let mut boards = Vec::with_capacity(input.len() / 75);

        for b in input[line_length + 2..].chunks(76) {
            let mut board = Board::default();
            let mut output = board.iter_mut().flatten();
            for c in b.chunks_exact(3) {
                match c {
                    [x @ b'0'..=b'9', y @ b'0'..=b'9', b' ' | b'\n'] => {
                        *output.next().unwrap() = (x - b'0') * 10 + (y - b'0');
                    }
                    [b' ', y @ b'0'..=b'9', b' ' | b'\n'] => {
                        *output.next().unwrap() = y - b'0';
                    }
                    _ => {
                        unreachable!();
                    }
                }
            }
            boards.push(board);
        }

        (calls, boards)
    }

    fn part1((calls, boards): Self::Parsed) -> Self::Output {
        let mut called = vec![[[false; 5]; 5]; boards.len()];

        let (winner, last_call) = 'w: {
            for c in calls {
                for i in 0..called.len() {
                    for x in 0..5 {
                        for y in 0..5 {
                            if boards[i][x][y] == c {
                                called[i][x][y] = true;

                                if called[i].iter().all(|r| r[y]) || called[i][x].iter().all(|x| *x)
                                {
                                    break 'w (i, c);
                                }
                            }
                        }
                    }
                }
            }
            unreachable!()
        };

        boards[winner]
            .iter()
            .flatten()
            .zip(called[winner].iter().flatten())
            .filter(|(_, &called)| !called)
            .map(|(&num, _)| num as u16)
            .sum::<u16>()
            * (last_call as u16)
    }

    fn part2((calls, boards): Self::Parsed) -> Self::Output {
        let mut called = vec![[[false; 5]; 5]; boards.len()];
        let mut won = vec![false; boards.len()];

        let (winner, last_call) = 'w: {
            for c in calls {
                for i in 0..called.len() {
                    if !won[i] {
                        for x in 0..5 {
                            for y in 0..5 {
                                if boards[i][x][y] == c {
                                    called[i][x][y] = true;

                                    if called[i].iter().all(|r| r[y])
                                        || called[i][x].iter().all(|x| *x)
                                    {
                                        won[i] = true;

                                        if won.iter().all(|&x| x) {
                                            break 'w (i, c);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            unreachable!()
        };

        boards[winner]
            .iter()
            .flatten()
            .zip(called[winner].iter().flatten())
            .filter(|(_, &called)| !called)
            .map(|(&num, _)| num as u16)
            .sum::<u16>()
            * (last_call as u16)
    }
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
