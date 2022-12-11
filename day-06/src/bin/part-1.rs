use std::{
    fs,
};

use itertools::Itertools;
use nom::{
    bytes::complete::take,
    character::complete::anychar,
    combinator::{fail, peek},
    multi::many0_count,
    IResult,
};

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let marker_position = part_1(&input).unwrap();
    println!("Marker Position {}", marker_position);
}

fn part_1(input: &str) -> Result<usize, nom::Err<nom::error::Error<&str>>> {
    let (_, marker_position) =
        nom::combinator::map(many0_count(is_not_marker_start), |position| 
        // add 4 because is_not_marker_start looks at next 4 chars and returns the 1st one if it's not a marker. So we actually find the 0 index of the 1st marker. And then need to shift to find the last position for the question
        position + 4)(input)?;
    Ok(marker_position)
}

fn is_not_marker_start(input: &str) -> IResult<&str, char> {
    let (input, next_3) = peek(take(4usize))(input)?;
    if next_3.chars().all_unique() {
        fail(input)
    } else {
        anychar(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT_1: &'static str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    const TEST_INPUT_2: &'static str = "bvwbjplbgvbhsrlpgdmjqwftvncz";
    const TEST_INPUT_3: &'static str = "nppdvjthqldpwncqszvftbrmjlhg";
    const TEST_INPUT_4: &'static str = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
    const TEST_INPUT_5: &'static str = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";

    #[test]
    fn part_1_test_input_1() {
        assert_eq!(part_1(TEST_INPUT_1).unwrap(), 7);
    }

    #[test]
    fn part_1_test_input_2() {
        assert_eq!(part_1(TEST_INPUT_2).unwrap(), 5);
    }

    #[test]
    fn part_1_test_input_3() {
        assert_eq!(part_1(TEST_INPUT_3).unwrap(), 6);
    }

    #[test]
    fn part_1_test_input_4() {
        assert_eq!(part_1(TEST_INPUT_4).unwrap(), 10);
    }

    #[test]
    fn part_1_test_input_5() {
        assert_eq!(part_1(TEST_INPUT_5).unwrap(), 11);
    }
}
