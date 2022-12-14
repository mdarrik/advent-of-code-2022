use std::{collections::VecDeque, fs};

use day_11::{parse_input, Monkey};

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let monkey_business = part_2(&input).unwrap();
    println!("Level of monkey business is {}", monkey_business);
}

fn part_2(input: &str) -> Result<u64, nom::Err<nom::error::Error<&str>>> {
    let (_, mut monkeys) = parse_input(input)?;
    monkeys.sort_by_key(|m| m.identifier);
    let common_remainder = monkeys.iter().map(|monkey| monkey.test_number).product();
    let mut monkeys: VecDeque<Monkey> = VecDeque::from(monkeys);
    for _ in 0..10000 {
        for _ in 0..monkeys.len() {
            if let Some(mut current_monkey) = monkeys.pop_front() {
                current_monkey.take_turn_part_2(&mut monkeys, common_remainder);
                monkeys.push_back(current_monkey);
            }
        }
    }
    let mut monkey_counts = monkeys.iter().map(|m| m.inspection_count).collect::<Vec<_>>();
    monkey_counts.sort();
    let monkey_business = monkey_counts.iter().rev().take(2).product();
    
    Ok(monkey_business)
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &'static str = include_str!("../../test-input.txt");
    #[test]
    fn part_2_test() {
        assert_eq!(part_2(TEST_INPUT).unwrap(), 2713310158);
    }
}
