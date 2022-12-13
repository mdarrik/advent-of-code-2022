use std::fs;

use nom::{
    branch::alt, bytes::complete::tag, character::complete::newline, combinator::map,
    multi::separated_list1, IResult,
};

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let signal_strength = part_1(&input).unwrap();
    println!("signal strength is {}", signal_strength);
}

fn part_1(input: &str) -> Result<i32, nom::Err<nom::error::Error<&str>>> {
    let (_, instructions) = parse_input(input)?;
    let mut cycles_of_interest = (20..=220).step_by(40);
    dbg!(&cycles_of_interest.clone().collect::<Vec<_>>());
    let mut cycle_to_monitor = cycles_of_interest.next();
    let mut current_cycle = 1;
    let mut instruction_iter = instructions.iter();
    let mut x_register = 1;
    let mut signal_values = Vec::with_capacity(6);
    while cycle_to_monitor.is_some() {
        if let Some(cycle_of_interest) = cycle_to_monitor {
            if let Some(instruction) = instruction_iter.next() {
                match instruction {
                    Instruction::Noop => {
                        current_cycle += 1;
                    }
                    Instruction::AddX(amount) => {
                        if current_cycle + 2 > cycle_of_interest {
                            dbg!(current_cycle, cycle_of_interest, x_register);
                            signal_values.push(cycle_of_interest * x_register);
                            cycle_to_monitor = cycles_of_interest.next();
                        }
                        current_cycle += 2;
                        x_register += amount;
                    }
                }
            }

            if current_cycle == cycle_of_interest {
                signal_values.push(cycle_of_interest * x_register);
                cycle_to_monitor = cycles_of_interest.next();
            }
        }
    }
    dbg!(&signal_values, x_register);
    let sum_of_signals = signal_values.into_iter().sum::<i32>();
    Ok(sum_of_signals)
}

fn parse_input(input: &str) -> IResult<&str, Vec<Instruction>> {
    separated_list1(newline, instruction)(input)
}

fn instruction(input: &str) -> IResult<&str, Instruction> {
    alt((noop, addx))(input)
}

fn noop(input: &str) -> IResult<&str, Instruction> {
    map(tag("noop"), |_| Instruction::Noop)(input)
}

fn addx(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag("addx ")(input)?;
    let (input, val) = nom::character::complete::i32(input)?;

    Ok((input, Instruction::AddX(val)))
}

enum Instruction {
    Noop,
    AddX(i32),
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &'static str = include_str!("../../test-input.txt");
    #[test]
    fn part_1_test() {
        assert_eq!(part_1(TEST_INPUT).unwrap(), 13140);
    }
}
