use nom::{
    branch::alt, bytes::complete::tag, character::complete::newline, combinator::map,
    multi::separated_list1, IResult,
};

pub fn parse_input(input: &str) -> IResult<&str, Vec<Instruction>> {
    separated_list1(newline, instruction)(input)
}

fn instruction(input: &str) -> IResult<&str, Instruction> {
    alt((noop, addx))(input)
}

fn noop(input: &str) -> IResult<&str, Instruction> {
    map(tag("noop"), |_| Instruction::Noop)(input)
}

fn addx(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag("addx ")(input)?;
    let (input, val) = nom::character::complete::i32(input)?;

    Ok((input, Instruction::AddX(val)))
}

pub enum Instruction {
    Noop,
    AddX(i32),
}
