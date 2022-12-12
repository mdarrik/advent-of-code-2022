use std::{collections::HashSet, fs};

use day_09::{parse_input, Motion, Position};
use itertools::Itertools;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let points_visited = part_1(&input).unwrap();
    println!("number of points the tail visited: {}", points_visited);
}

fn part_1(input: &str) -> Result<usize, nom::Err<nom::error::Error<&str>>> {
    let (_, commands) = parse_input(input)?;
    let mut positions = vec![Position::default(); 10];
    let mut visited_map = HashSet::new();
    visited_map.insert(*positions.last().unwrap());
    for command in commands {
        match command {
            Motion::Up(steps) => {
                for _ in 0..steps {
                    positions[0].y += 1;
                    move_rope(&mut positions);
                    visited_map.insert(*positions.last().unwrap());
                }
            }
            Motion::Down(steps) => {
                for _ in 0..steps {
                    positions[0].y -= 1;
                    move_rope(&mut positions);
                    visited_map.insert(*positions.last().unwrap());
                }
            }
            Motion::Left(steps) => {
                for _ in 0..steps {
                    positions[0].x += 1;
                    move_rope(&mut positions);
                    visited_map.insert(*positions.last().unwrap());
                }
            }
            Motion::Right(steps) => {
                for _ in 0..steps {
                    positions[0].x -= 1;
                    move_rope(&mut positions);
                    visited_map.insert(*positions.last().unwrap());
                }
            }
        }
    }
    Ok(visited_map.len())
}

fn move_rope(positions: &mut Vec<Position>) {
    for (head_index, tail_index) in (0..(positions.len())).tuple_windows() {
        let head = positions[head_index];
        let tail = positions[tail_index];
        positions[tail_index] = move_tail(&head, &tail);
    }
}

fn move_tail(new_head_position: &Position, tail_position: &Position) -> Position {
    if new_head_position.touching(tail_position) {
        return *tail_position;
    }
    let mut new_position = *tail_position;
    let y_diff = new_head_position.y - tail_position.y;
    let x_diff = new_head_position.x - tail_position.x;
    match y_diff.signum() {
        -1 => new_position.y -= 1,
        1 => new_position.y += 1,
        _ => (),
    }
    match x_diff.signum() {
        -1 => new_position.x -= 1,
        1 => new_position.x += 1,
        _ => (),
    }
    new_position
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT_1: &'static str = include_str!("../../test-input-1.txt");
    const TEST_INPUT_2: &'static str = include_str!("../../test-input-2.txt");
    #[test]
    fn part_2_test_1() {
        assert_eq!(part_1(TEST_INPUT_1).unwrap(), 1);
    }

    #[test]
    fn part_2_test_2() {
        assert_eq!(part_1(TEST_INPUT_2).unwrap(), 36);
    }
}
