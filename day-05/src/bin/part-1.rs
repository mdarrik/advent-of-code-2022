use std::fs;

use day_05::parse_input;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let crates_on_top = part_1(&input).unwrap();
    println!("The crates on top are {}", crates_on_top);
}

fn part_1(input: &str) -> Result<String, nom::Err<nom::error::Error<&str>>> {
    let (_, (mut stacks, instructions)) = parse_input(input)?;

    for instruction in instructions.iter() {
        let mut moving_crates: Vec<char> = vec![];
        if let Some(stack) = stacks.get_mut((instruction.from as usize) - 1) {
            moving_crates
                .extend(stack.drain((stack.len() - (instruction.count as usize))..stack.len()));
        }
        if let Some(stack) = stacks.get_mut((instruction.to as usize) - 1) {
            moving_crates.reverse();
            stack.extend(moving_crates);
        }
    }

    let chars = stacks.iter().fold(String::new(), |mut elements, stack| {
        if let Some(el) = stack.last() {
            elements.push(*el);
        }
        elements
    });

    Ok(chars)
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &'static str = include_str!("../../test-input.txt");
    #[test]
    fn part_1_test() {
        assert_eq!(part_1(TEST_INPUT).unwrap(), "CMZ");
    }
}
