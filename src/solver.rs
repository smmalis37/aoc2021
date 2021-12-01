use std::fmt::Debug;

pub trait Solver<'a> {
    type Parsed: Clone;
    type Output: Debug + PartialEq;
    type Output2: Debug + PartialEq = Self::Output;

    fn parse(input: &'a str) -> Self::Parsed;
    fn part1(data: Self::Parsed) -> Self::Output;
    fn part2(data: Self::Parsed) -> Self::Output2;
}
