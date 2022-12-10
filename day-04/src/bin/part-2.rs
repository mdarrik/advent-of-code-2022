use std::fs;

use day_04::parse_input;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let number_of_overlaps = part_2(&input).unwrap();
    println!("There are {} overlaps", number_of_overlaps);
}

fn part_2(input: &str) -> Result<u32, nom::Err<nom::error::Error<&str>>> {
    let (_, pairs) = parse_input(input)?;
    Ok(pairs
        .iter()
        .filter(|(elf_1, elf_2)| {
            elf_1.contains(elf_2.start())
                || elf_1.contains(elf_2.end())
                || elf_2.contains(elf_1.start())
                || elf_2.contains(elf_1.end())
        })
        .count() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &'static str = include_str!("../../test-input.txt");
    #[test]
    fn part_2_test() {
        assert_eq!(part_2(TEST_INPUT).unwrap(), 4);
    }
}
