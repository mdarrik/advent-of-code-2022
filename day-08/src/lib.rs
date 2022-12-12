use ndarray::Array2;
use nom::{
    character::complete::{anychar, newline},
    combinator::{map, verify},
    multi::{many1, separated_list0},
    IResult,
};

pub fn parse_input(input: &str) -> IResult<&str, Array2<u8>> {
    let (input, trees) = separated_list0(
        newline,
        many1(map(verify(anychar, |c| c.is_ascii_digit()), |c| {
            u8::try_from(c.to_digit(10).unwrap()).unwrap()
        })),
    )(input)?;
    let tree_matrix: Array2<u8> = Array2::from_shape_vec(
        (trees.len(), trees[0].len()),
        trees.into_iter().flatten().collect::<Vec<_>>(),
    )
    .unwrap();
    Ok((input, tree_matrix))
}
