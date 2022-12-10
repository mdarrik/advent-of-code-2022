use std::ops::RangeInclusive;

use nom::{
    bytes::complete::tag, character::complete::newline, multi::separated_list1,
    sequence::separated_pair, IResult,
};

pub type ElfRange = RangeInclusive<u32>;

pub fn parse_input(input: &str) -> IResult<&str, Vec<(ElfRange, ElfRange)>> {
    separated_list1(newline, pairs)(input)
}

fn pairs(input: &str) -> IResult<&str, (ElfRange, ElfRange)> {
    separated_pair(elf_range, tag(","), elf_range)(input)
}

fn elf_range(input: &str) -> IResult<&str, ElfRange> {
    let (input, start) = nom::character::complete::u32(input)?;
    let (input, _) = tag("-")(input)?;
    let (input, end) = nom::character::complete::u32(input)?;
    Ok((input, RangeInclusive::new(start, end)))
}
