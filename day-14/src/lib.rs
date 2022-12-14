use std::fmt::Display;

use nom::{
    bytes::complete::tag, character::complete::newline, combinator::map, multi::separated_list1,
    sequence::separated_pair, IResult,
};

pub fn parse_input(input: &str) -> IResult<&str, Vec<Vec<Point>>> {
    separated_list1(newline, lines)(input)
}

fn lines(input: &str) -> IResult<&str, Vec<Point>> {
    separated_list1(tag(" -> "), point)(input)
}

fn point(input: &str) -> IResult<&str, Point> {
    map(
        separated_pair(
            nom::character::complete::u32,
            tag(","),
            nom::character::complete::u32,
        ),
        |(x, y)| Point {
            x: x as usize,
            y: y as usize,
        },
    )(input)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Default for Point {
    fn default() -> Self {
        Self { x: 500, y: 0 }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CaveSpace {
    #[default]
    Air,
    Rock,
    Sand,
}

impl Display for CaveSpace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let character = match self {
            CaveSpace::Rock => '#',
            CaveSpace::Air => '.',
            CaveSpace::Sand => '*',
        };
        write!(f, "{character}")
    }
}
