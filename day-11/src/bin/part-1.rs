use std::{collections::VecDeque, fs};

use day_11::parse_input;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let monkey_business = part_1(&input).unwrap();
    println!("Level of monkey business is {}", monkey_business);
}

fn part_1(input: &str) -> Result<u64, nom::Err<nom::error::Error<&str>>> {
    let (_, mut monkeys) = parse_input(input)?;
    monkeys.sort_by_key(|m| m.identifier);
    let mut monkeys = VecDeque::from(monkeys);
    for _ in 0..20 {
        for _ in 0..monkeys.len() {
            if let Some(mut current_monkey) = monkeys.pop_front() {
                current_monkey.take_turn_part_1(&mut monkeys);
                monkeys.push_back(current_monkey);
            }
        }
    }
    let most_active_monkey = monkeys.iter().max_by_key(|m| m.inspection_count).unwrap();
    let second_most_active_monkey = monkeys
        .iter()
        .reduce(|max_activity_monkey, monkey| {
            if monkey.inspection_count > max_activity_monkey.inspection_count
                && monkey.identifier != most_active_monkey.identifier
            {
                monkey
            } else {
                max_activity_monkey
            }
        })
        .unwrap();

    Ok(most_active_monkey.inspection_count * second_most_active_monkey.inspection_count)
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &'static str = include_str!("../../test-input.txt");
    #[test]
    fn part_1_test() {
        assert_eq!(part_1(TEST_INPUT).unwrap(), 10605);
    }
}
