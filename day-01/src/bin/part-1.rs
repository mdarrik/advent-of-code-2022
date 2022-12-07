use std::fs;

use day_01::parse_input;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let response = find_most_calories(&input).unwrap();
    println!("Max calories is {}", response);
}

fn find_most_calories(input: &str) -> Result<u32, nom::Err<nom::error::Error<&str>>> {
    println!("{:?}", &input);
    let (_, elf_calories) = parse_input(input)?;
    println!("{:?}", &elf_calories);
    let max_colories = elf_calories
        .iter()
        .map(|v| v.iter().sum::<u32>())
        .collect::<Vec<u32>>()
        .into_iter()
        .max()
        .unwrap();
    Ok(max_colories)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &'static str = include_str!("../../test-input.txt");

    #[test]
    fn part_1() {
        assert_eq!(find_most_calories(TEST_INPUT).unwrap(), 24_000)
    }
}
