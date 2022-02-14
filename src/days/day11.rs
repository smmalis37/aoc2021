use crate::{solver::Solver, util::*};

use super::day9::Day9;

const NEIGHBORS: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

const FLASHED: u8 = b':';

pub struct Day11;

impl<'a> Solver<'a> for Day11 {
    type Parsed = Grid<u8>;
    type Output = u32;

    fn parse(input: &'a str) -> Self::Parsed {
        <Day9 as Solver>::parse(input)
    }

    fn part1(mut data: Self::Parsed) -> Self::Output {
        (0..100).map(|_| do_step(&mut data)).sum()
    }

    fn part2(mut data: Self::Parsed) -> Self::Output {
        for i in 1.. {
            if do_step(&mut data) as usize == data.len() {
                return i;
            }
        }
        unreachable!()
    }
}

#[inline]
fn do_step<'a>(data: &mut <Day11 as Solver>::Parsed) -> <Day11 as Solver<'a>>::Output {
    let mut flashes = 0;

    for i in 0..data.line_count() {
        for j in 0..data.line_length() {
            flashes += flash(data, i, j);
        }
    }

    data.iter_mut()
        .flatten()
        .filter(|x| **x >= FLASHED)
        .for_each(|x| *x = b'0');

    flashes
}

#[inline]
fn flash<'a>(
    data: &mut <Day11 as Solver>::Parsed,
    i: usize,
    j: usize,
) -> <Day11 as Solver<'a>>::Output {
    let maybe = data.get_mut(i).and_then(|x| x.get_mut(j));
    if let Some(me) = maybe {
        *me += 1;
        if *me == FLASHED {
            return NEIGHBORS
                .iter()
                .map(|&(ni, nj)| flash(data, i.wrapping_add_signed(ni), j.wrapping_add_signed(nj)))
                .fold(1, |a, e| a + e);
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d11p1() {
        assert_eq!(
            Day11::part1(Day11::parse(
                "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526
"
            )),
            1656
        );
    }

    #[test]
    fn d11p2() {
        assert_eq!(
            Day11::part2(Day11::parse(
                "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526
"
            )),
            195
        );
    }
}
