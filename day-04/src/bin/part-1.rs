use std::fs;

use day_04::{parse_input, ElfRange};

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let number_of_overlaps = part_1(&input).unwrap();
    println!("There are {} overlaps", number_of_overlaps);
}

fn part_1(input: &str) -> Result<u32, nom::Err<nom::error::Error<&str>>> {
    let (_, pairs) = parse_input(input)?;
    Ok(pairs
        .iter()
        .filter(|(elf_1, elf_2)| {
            range_fully_contains(elf_1, elf_2) || range_fully_contains(elf_2, elf_1)
        })
        .count() as u32)
}

fn range_fully_contains(this_range: &ElfRange, range_to_check: &ElfRange) -> bool {
    this_range.contains(range_to_check.start()) && this_range.contains(range_to_check.end())
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &'static str = include_str!("../../test-input.txt");
    #[test]
    fn part_1_test() {
        assert_eq!(part_1(TEST_INPUT).unwrap(), 2);
    }
}
