use std::fs;

use day_10::{parse_input, Instruction};
use ndarray::Array2;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let display = part_1(&input).unwrap();
    println!("{}", display_array_as_string(&display));
}

fn part_1(input: &str) -> Result<Array2<char>, nom::Err<nom::error::Error<&str>>> {
    let (_, instructions) = parse_input(input)?;

    let mut instruction_iter = instructions.iter();
    let mut x_register = 1;

    let mut display_vec = ['.'; 240];
    let mut current_cycle = 0_u16;
    // draw for cycle 0
    update_display_vec(&mut display_vec, &x_register, &current_cycle);
    while current_cycle <= 240 {
        if let Some(instruction) = instruction_iter.next() {
            match instruction {
                Instruction::Noop => {
                    current_cycle += 1;
                }
                Instruction::AddX(add_amount) => {
                    current_cycle += 1;
                    update_display_vec(&mut display_vec, &x_register, &current_cycle);
                    current_cycle += 1;
                    x_register += add_amount;
                }
            }
        } else {
            break;
        }
        update_display_vec(&mut display_vec, &x_register, &current_cycle);
    }
    update_display_vec(&mut display_vec, &x_register, &current_cycle);

    Ok(Array2::from_shape_vec((6, 40), display_vec.to_vec()).unwrap())
}

fn update_display_vec(display_vec: &mut [char], x_register: &i32, cycle: &u16) {
    let cycle_row = cycle % 40;

    let display_buffer_range = u16::try_from(x_register - 1).unwrap_or_default()
        ..=u16::try_from(x_register + 1).unwrap_or_default();
    if display_buffer_range.contains(&cycle_row) {
        display_vec[usize::from(*cycle)] = '#';
    }
}

fn display_array_as_string(display_array: &Array2<char>) -> String {
    let (row_count, _) = display_array.dim();
    display_array.axis_iter(ndarray::Axis(0)).enumerate().fold(
        String::new(),
        |mut display, (current_row_num, row)| {
            row.iter().for_each(|e| display.push(*e));
            if current_row_num < row_count - 1 {
                display.push('\n');
            }
            display
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &'static str = include_str!("../../test-input.txt");
    const TEST_RESPONSE: &'static str = include_str!("../../test-input-part-2-response.txt");
    #[test]
    fn part_1_test() {
        let array_result = part_1(TEST_INPUT).unwrap();
        let s = display_array_as_string(&array_result);
        assert_eq!(s, TEST_RESPONSE);
    }
}
