use std::fs;

use day_01::parse_input;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let result = find_sum_top_3_calories(&input).unwrap();
    println!("sum of top 3 calories is {}", result);
}

fn find_sum_top_3_calories(input: &str) -> Result<u32, nom::Err<nom::error::Error<&str>>> {
    let (_, elf_calories) = parse_input(input)?;

    let mut calorie_sums = elf_calories
        .iter()
        .map(|elf| elf.iter().sum::<u32>())
        .collect::<Vec<u32>>();
    calorie_sums.sort_unstable();
    let top_3_sum = calorie_sums.iter().rev().take(3).sum::<u32>();
    println!("{:?}", top_3_sum);
    Ok(top_3_sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &'static str = include_str!("../../test-input.txt");

    #[test]
    fn part_2() {
        assert_eq!(find_sum_top_3_calories(TEST_INPUT).unwrap(), 45_000)
    }
}
