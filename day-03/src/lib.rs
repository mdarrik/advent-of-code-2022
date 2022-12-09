use nom::{
    character::complete::{alpha1, newline},
    multi::separated_list0,
    IResult,
};

pub fn calculate_priority(letter: char) -> u32 {
    if letter.is_lowercase() {
        letter as u32 - 'a' as u32 + 1
    } else {
        letter as u32 - 'A' as u32 + 27
    }
}

pub fn parse_input(input: &str) -> IResult<&str, Vec<&str>> {
    separated_list0(newline, alpha1)(input)
}
