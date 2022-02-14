use crate::{solver::Solver, util::*};

pub struct Day9;

impl<'a> Solver<'a> for Day9 {
    type Parsed = Grid<u8>;
    type Output = u32;

    fn parse(input: &'a str) -> Self::Parsed {
        let input = input.as_bytes();
        let line_length = memchr::memchr(b'\n', input).unwrap();
        let mut data = Vec::with_capacity(input.len());
        data.extend(
            input
                .chunks_exact(line_length + 1)
                .flat_map(|l| &l[..line_length])
                .copied(),
        );

        Grid::from_vec(data, line_length, input.len() / (line_length + 1))
    }

    fn part1(data: Self::Parsed) -> Self::Output {
        let mut sum = 0;

        for r in 0..data.line_count() {
            for c in 0..data.line_length() {
                let check = |ra, ca| {
                    r.checked_add_signed(ra)
                        .zip(c.checked_add_signed(ca))
                        .and_then(|(r2, c2)| data.get(r2).and_then(|r3| r3.get(c2)))
                        .map_or(true, |&x| data[r][c] < x)
                };
                if check(0, -1) && check(0, 1) && check(-1, 0) && check(1, 0) {
                    sum += (data[r][c] + 1 - b'0') as Self::Output;
                }
            }
        }

        sum
    }

    #[allow(clippy::stable_sort_primitive)] // It's faster here because basins is already mostly sorted.
    fn part2(mut data: Self::Parsed) -> Self::Output {
        let mut stack = Vec::with_capacity(data.len());
        let mut basins = [0; 4];

        for r in 0..data.line_count() {
            for c in 0..data.line_length() {
                if data[r][c] != b'9' {
                    stack.push((r, c));
                    let mut count = 0;

                    while let Some((sr, sc)) = stack.pop() {
                        let cell = data.get(sr).and_then(|x| x.get(sc));
                        if cell.is_some() && cell != Some(&b'9') {
                            data[sr][sc] = b'9';
                            count += 1;
                            stack.extend_from_slice(&[
                                (sr, sc + 1),
                                (sr, sc.wrapping_add_signed(-1)),
                                (sr + 1, sc),
                                (sr.wrapping_add_signed(-1), sc),
                            ]);
                        }
                    }

                    basins[0] = count;
                    basins.sort();
                }
            }
        }

        basins.into_iter().skip(1).product()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d9p1() {
        assert_eq!(
            Day9::part1(Day9::parse(
                "2199943210
3987894921
9856789892
8767896789
9899965678
"
            )),
            15
        );
    }

    #[test]
    fn d9p2() {
        assert_eq!(
            Day9::part2(Day9::parse(
                "2199943210
3987894921
9856789892
8767896789
9899965678
"
            )),
            1134
        );
    }
}
