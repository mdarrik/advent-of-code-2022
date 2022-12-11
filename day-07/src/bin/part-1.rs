use std::fs;

use day_07::parse_input;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let size_sum = part_1(&input).unwrap();
    println!("sum of dir size greather than 1000 is {}", size_sum);
}

fn part_1(input: &str) -> Result<u64, nom::Err<nom::error::Error<&str>>> {
    let (_, size_map) = parse_input(input)?;

    let total_size_of_dirs_greater_than_10000 =
        size_map
            .values()
            .fold(0, |dirs_greater_than_10000, dir_size| {
                if *dir_size <= 100_000 {
                    return dirs_greater_than_10000 + *dir_size;
                }
                dirs_greater_than_10000
            });

    Ok(total_size_of_dirs_greater_than_10000)
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &'static str = include_str!("../../test-input.txt");
    #[test]
    fn part_1_test() {
        assert_eq!(part_1(TEST_INPUT).unwrap(), 95437);
    }
}
