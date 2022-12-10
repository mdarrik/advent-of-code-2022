use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{anychar, newline, not_line_ending, space1},
    multi::separated_list0,
    sequence::delimited,
    IResult,
};

pub fn parse_input(input: &str) -> IResult<&str, (ShippingStacks, Vec<Instruction>)> {
    let (input, rows) = separated_list0(
        newline,
        separated_list0(tag(" "), alt((shipping_crate, empty_slot))),
    )(input)?;
    let (input, _) = not_line_ending(input)?;
    let (input, _) = newline(input)?;
    let (input, _) = newline(input)?;
    let (input, instructions) = separated_list0(newline, instruction)(input)?;
    let stacks = rows
        .iter()
        .rev()
        .fold(vec![vec![]; rows[0].len()], |mut stacks, row| {
            row.iter()
                .enumerate()
                .for_each(|(position, shipping_crate)| {
                    if let Some(id) = shipping_crate {
                        stacks.get_mut(position).unwrap().push(*id);
                    }
                });
            stacks
        });
    Ok((input, (stacks, instructions)))
}

fn empty_slot(input: &str) -> IResult<&str, Option<char>> {
    let (input, _) = tag("   ")(input)?;
    Ok((input, None))
}

fn shipping_crate(input: &str) -> IResult<&str, Option<char>> {
    let (input, crate_identifier) = delimited(tag("["), anychar, tag("]"))(input)?;
    Ok((input, Some(crate_identifier)))
}

fn instruction(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag("move ")(input)?;
    let (input, count) = nom::character::complete::u8(input)?;
    let (input, _) = space1(input)?;
    let (input, _) = tag("from ")(input)?;
    let (input, from) = nom::character::complete::u8(input)?;
    let (input, _) = space1(input)?;
    let (input, _) = tag("to ")(input)?;
    let (input, to) = nom::character::complete::u8(input)?;
    Ok((input, Instruction { count, from, to }))
}

#[derive(Debug, Copy, Clone)]
pub struct Instruction {
    pub count: u8,
    pub from: u8,
    pub to: u8,
}

pub type ShippingStacks = Vec<Vec<char>>;
