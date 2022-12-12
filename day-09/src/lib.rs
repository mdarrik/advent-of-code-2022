use nom::{
    character::complete::{newline, one_of, space1},
    multi::separated_list1,
    IResult,
};

pub fn parse_input(input: &str) -> IResult<&str, Vec<Motion>> {
    separated_list1(newline, motion)(input)
}

fn motion(input: &str) -> IResult<&str, Motion> {
    let (input, motion_direction) = one_of("UDLR")(input)?;
    let (input, _) = space1(input)?;
    let (input, number_of_steps) = nom::character::complete::u16(input)?;

    let motion = match motion_direction {
        'U' => Motion::Up(number_of_steps),
        'D' => Motion::Down(number_of_steps),
        'L' => Motion::Left(number_of_steps),
        'R' => Motion::Right(number_of_steps),
        _ => panic!("Invaid command reached"),
    };
    Ok((input, motion))
}

#[derive(Debug, Clone, Copy)]
pub enum Motion {
    Up(u16),
    Down(u16),
    Left(u16),
    Right(u16),
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn touching(&self, other: &Position) -> bool {
        self == other || (self.x.abs_diff(other.x) <= 1 && self.y.abs_diff(other.y) <= 1)
    }
}
