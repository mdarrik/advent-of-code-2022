use std::{collections::HashMap, fs};

use day_03::{calculate_priority, parse_input};

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    println!("priority of items is {}", part_1(&input).unwrap());
}

fn part_1(input: &str) -> Result<u32, nom::Err<nom::error::Error<&str>>> {
    let (_, packs) = parse_input(input)?;
    let priority = packs.iter().fold(0, |mut total_priority, pack| {
        let mut item_tracker: HashMap<char, bool> = HashMap::new();
        let pack_chars = pack.chars().collect::<Vec<char>>();
        let (compartment_1, compartment_2) = pack_chars.split_at(pack_chars.len() / 2);
        item_tracker.extend(compartment_1.iter().map(|e| (e, &true)));
        compartment_2.iter().for_each(|v| {
            let is_scoreable = *item_tracker.get(v).unwrap_or(&false);
            if is_scoreable {
                total_priority += calculate_priority(*v);
                item_tracker.entry(*v).and_modify(|value| {
                    *value = false;
                });
            }
        });
        total_priority
    });
    Ok(priority)
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &'static str = include_str!("../../test-input.txt");
    #[test]
    fn part_1_test() {
        assert_eq!(part_1(TEST_INPUT).unwrap(), 157);
    }
}
