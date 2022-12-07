use nom::{
    character::complete::newline, combinator::opt, multi::separated_list0, sequence::tuple, IResult,
};

pub fn parse_input(input: &str) -> IResult<&str, Vec<Vec<u32>>> {
    let (input, _) = opt(newline)(input)?;
    // let (input, test_value) = separated_list0(newline, nom::character::complete::u16)(input)?;
    // println!("{:?}", test_value);
    separated_list0(
        tuple((newline, newline)),
        separated_list0(newline, nom::character::complete::u32),
    )(input)

    // Ok((input, vec![]));
}
