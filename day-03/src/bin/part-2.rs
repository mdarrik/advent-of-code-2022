use std::{collections::HashSet, fs};

use day_03::{calculate_priority, parse_input};
use itertools::Itertools;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    println!("priority of badges is {}", part_2(&input).unwrap());
}

fn part_2(input: &str) -> Result<u32, nom::Err<nom::error::Error<&str>>> {
    let (_, data) = parse_input(input)?;

    let data = &data.iter().chunks(3).into_iter().fold(0, |score, group| {
        let letters = group
            .map(|row| row.chars().collect::<HashSet<char>>())
            .reduce(|reducer, row| reducer.intersection(&row).map(|c| *c).collect())
            .unwrap_or_default();
        score + calculate_priority(*letters.iter().next().unwrap())
    });
    Ok(*data)
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &'static str = include_str!("../../test-input.txt");
    #[test]
    fn part_1_test() {
        assert_eq!(part_2(TEST_INPUT).unwrap(), 70);
    }
}
