use std::{collections::HashSet, fs};

use day_09::{Position, parse_input, Motion};

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let points_visited = part_1(&input).unwrap();
    println!("number of points the tail visited: {}", points_visited);
}

fn part_1(input: &str) -> Result<usize, nom::Err<nom::error::Error<&str>>> {
    let (_, commands) = parse_input(input)?;
    let mut tail_position = Position::default();
    let mut head_position = Position::default();
    let mut visited_map = HashSet::new();
    visited_map.insert(tail_position);
    commands.iter().for_each(|command| match command {
        Motion::Up(steps) => {
            for _ in 0..*steps {
                head_position.y += 1;
                move_tail(&head_position, &mut tail_position);
                visited_map.insert(tail_position);
            }
        }
        Motion::Down(steps) => {
            for _ in 0..*steps {
                head_position.y -= 1;
                move_tail(&head_position, &mut tail_position);
                visited_map.insert(tail_position);
            }
        }
        Motion::Left(steps) => {
            for _ in 0..*steps {
                head_position.x += 1;
                move_tail(&head_position, &mut tail_position);
                visited_map.insert(tail_position);
            }
        }
        Motion::Right(steps) => {
            for _ in 0..*steps {
                head_position.x -= 1;
                move_tail(&head_position, &mut tail_position);
                visited_map.insert(tail_position);
            }
        }
    });
    Ok(visited_map.len())
}

fn move_tail(new_head_position: &Position, tail_position: &mut Position) {
    if new_head_position.touching(tail_position) {
        return;
    }
    let y_diff = new_head_position.y - tail_position.y;
    let x_diff = new_head_position.x - tail_position.x;
    match y_diff.signum() {
        -1 => tail_position.y -= 1,
        1 => tail_position.y += 1,
        _ => (),
    }
    match x_diff.signum() {
        -1 => tail_position.x -= 1,
        1 => tail_position.x += 1,
        _ => (),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &'static str = include_str!("../../test-input-1.txt");
    #[test]
    fn part_1_test() {
        assert_eq!(part_1(TEST_INPUT).unwrap(), 13);
    }
}
