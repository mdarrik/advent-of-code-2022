use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{newline, one_of, space0, space1},
    combinator::{map, opt},
    multi::separated_list1,
    sequence::{delimited, preceded, tuple},
    IResult,
};
use std::collections::VecDeque;

pub fn parse_input(input: &str) -> IResult<&str, Vec<Monkey>> {
    separated_list1(newline, monkey)(input)
}

fn monkey(input: &str) -> IResult<&str, Monkey> {
    let (input, _) = tag("Monkey ")(input)?;
    let (input, id) = nom::character::complete::u8(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, _) = newline(input)?;
    let (input, _) = preceded(space1, tag("Starting items: "))(input)?;
    let (input, items) = map(
        separated_list1(tag(", "), nom::character::complete::u64),
        VecDeque::from,
    )(input)?;
    let (input, _) = newline(input)?;
    let (input, _) = preceded(space1, tag("Operation: new = "))(input)?;
    let (input, (first_operation_value, operator, second_operation_value)) =
        operation_expression(input)?;
    let (input, _) = newline(input)?;
    let (input, test_number) = preceded(
        preceded(space1, tag("Test: divisible by ")),
        nom::character::complete::u64,
    )(input)?;
    let (input, _) = newline(input)?;
    let (input, true_target_id) = preceded(
        preceded(space1, tag("If true: throw to monkey ")),
        nom::character::complete::u8,
    )(input)?;
    let (input, _) = newline(input)?;
    let (input, false_target_id) = preceded(
        preceded(space1, tag("If false: throw to monkey ")),
        nom::character::complete::u8,
    )(input)?;
    let (input, _) = opt(newline)(input)?;

    let new_monkey = Monkey {
        identifier: usize::from(id),
        items_list: items,
        operation_values: [first_operation_value, second_operation_value],
        operation_type: operator,
        test_number,
        true_target_id: usize::from(true_target_id),
        false_target_id: usize::from(false_target_id),
        inspection_count: 0,
    };
    Ok((input, new_monkey))
}

fn operation_expression(input: &str) -> IResult<&str, (OperationValue, Operation, OperationValue)> {
    tuple((operation_value, operation, operation_value))(input)
}

fn operation(input: &str) -> IResult<&str, Operation> {
    map(delimited(space0, one_of("*+"), space0), |c| match c {
        '+' => Operation::Addition,
        '*' => Operation::Multiplication,
        _ => panic!("Invalid operation character passed test"),
    })(input)
}

fn operation_value(input: &str) -> IResult<&str, OperationValue> {
    alt((operation_value_old, operation_value_number))(input)
}

fn operation_value_old(input: &str) -> IResult<&str, OperationValue> {
    map(delimited(space0, tag("old"), space0), |_| {
        OperationValue::Old
    })(input)
}

fn operation_value_number(input: &str) -> IResult<&str, OperationValue> {
    map(
        delimited(space0, nom::character::complete::u64, space0),
        OperationValue::Number,
    )(input)
}

#[derive(Default, Debug, Clone)]
pub struct Monkey {
    pub identifier: usize,
    pub operation_values: [OperationValue; 2],
    pub operation_type: Operation,
    pub test_number: u64,
    pub items_list: VecDeque<u64>,
    pub true_target_id: usize,
    pub false_target_id: usize,
    pub inspection_count: u64,
}

impl Monkey {
    pub fn update_worry(&self, worry: u64) -> u64 {
        let first_value = match self.operation_values[0] {
            OperationValue::Old => worry,
            OperationValue::Number(v) => v,
        };
        let second_value = match self.operation_values[1] {
            OperationValue::Old => worry,
            OperationValue::Number(v) => v,
        };

        match self.operation_type {
            Operation::Multiplication => first_value * second_value,
            Operation::Addition => first_value + second_value,
        }
    }

    pub fn take_turn_part_1(&mut self, monkeys: &mut VecDeque<Monkey>) {
        while let Some(item) = self.items_list.pop_front() {
            self.inspect_item_part_1(item, monkeys);
            self.inspection_count += 1;
        }
    }

    pub fn take_turn_part_2(&mut self, monkeys: &mut VecDeque<Monkey>, common_multiple: u64) {
        while let Some(item) = self.items_list.pop_front() {
            self.inspect_item_part_2(item, monkeys, common_multiple);
            self.inspection_count += 1;
        }
    }

    pub fn inspect_item_part_2(
        &mut self,
        item: u64,
        monkeys: &mut VecDeque<Monkey>,
        common_multiple: u64,
    ) {
        let new_worry = self.update_worry(item);
        if new_worry % self.test_number == 0 {
            if let Some(true_monkey) = monkeys
                .iter_mut()
                .find(|m| m.identifier == self.true_target_id)
            {
                true_monkey
                    .items_list
                    .push_back(new_worry % common_multiple);
            } else {
                panic!("couldn't find true monkey {}", self.true_target_id);
            }
        } else if let Some(false_monkey) = monkeys
            .iter_mut()
            .find(|m| m.identifier == self.false_target_id)
        {
            false_monkey
                .items_list
                .push_back(new_worry % common_multiple);
        }
    }

    pub fn inspect_item_part_1(&mut self, item: u64, monkeys: &mut VecDeque<Monkey>) {
        let new_worry = self.update_worry(item) / 3;
        if new_worry % self.test_number == 0 {
            if let Some(true_monkey) = monkeys
                .iter_mut()
                .find(|m| m.identifier == self.true_target_id)
            {
                true_monkey.items_list.push_back(new_worry);
            } else {
                panic!("couldn't find true monkey {}", self.true_target_id);
            }
        } else if let Some(false_monkey) = monkeys
            .iter_mut()
            .find(|m| m.identifier == self.false_target_id)
        {
            false_monkey.items_list.push_back(new_worry);
        }
    }
}

#[derive(Default, Debug, Clone, Copy)]
pub enum Operation {
    #[default]
    Multiplication,
    Addition,
}

#[derive(Default, Debug, Clone, Copy)]
pub enum OperationValue {
    #[default]
    Old,
    Number(u64),
}
