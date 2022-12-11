use std::fs;

use day_07::parse_input;

const UPDATE_SIZE: u64 = 30000000;
const TOTAL_SIZE: u64 = 70000000;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let dir_size = part_2(&input).unwrap();
    println!("dir size to delete is {}", dir_size);
}

fn part_2(input: &str) -> Result<u64, nom::Err<nom::error::Error<&str>>> {
    let (_, size_map) = parse_input(input)?;

    let max_size = *size_map.values().max().unwrap_or(&0);

    let size_to_clear = std::cmp::min(
        UPDATE_SIZE,
        UPDATE_SIZE.saturating_sub(TOTAL_SIZE.saturating_sub(max_size)),
    );

    let min_dir_size_to_delete = size_map
        .values()
        .filter_map(|v| if *v >= size_to_clear { Some(*v) } else { None })
        .min()
        .unwrap_or_default();

    Ok(min_dir_size_to_delete)
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &'static str = include_str!("../../test-input.txt");
    #[test]
    fn part_2_test() {
        assert_eq!(part_2(TEST_INPUT).unwrap(), 24_933_642);
    }
}
