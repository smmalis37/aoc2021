use crate::solver::Solver;

pub struct Day1;

type Num = u32;

impl<'a> Solver<'a> for Day1 {
    type Parsed = impl Iterator<Item = Num> + Clone;
    type Output = Num;

    fn parse(input: &'a str) -> Self::Parsed {
        let mut input = input.as_bytes();
        std::iter::from_fn(move || {
            (!input.is_empty()).then(|| {
                let (num, size) = lexical_core::parse_partial(input).unwrap();
                input = &input[size + 1..];
                num
            })
        })
    }

    fn part1(data: Self::Parsed) -> Self::Output {
        solve::<1>(data)
    }

    fn part2(data: Self::Parsed) -> Self::Output {
        solve::<3>(data)
    }
}

fn solve<const N: usize>(mut data: <Day1 as Solver>::Parsed) -> Num {
    let mut count = 0;
    let mut window = [0; N];

    for (w, d) in window.iter_mut().zip(&mut data) {
        *w = d;
    }

    for x in data {
        count += (x > window[0]) as Num;
        window.rotate_left(1);
        window[N - 1] = x;
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d1p1() {
        assert_eq!(
            Day1::part1(Day1::parse(
                "199
200
208
210
200
207
240
269
260
263
"
            )),
            7
        );
    }

    #[test]
    fn d1p2() {
        assert_eq!(
            Day1::part2(Day1::parse(
                "199
200
208
210
200
207
240
269
260
263
"
            )),
            5
        );
    }
}
