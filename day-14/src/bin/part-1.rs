use std::{
    cmp::{max, min},
    fs,
};

use day_14::{parse_input, CaveSpace, Point};
use itertools::Itertools;
use ndarray::Array2;

const PADDING_VALUE: usize = 2;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let units_of_sand = part_1(&input).unwrap();
    println!("units of sand {}", units_of_sand);
}

fn part_1(input: &str) -> Result<u32, nom::Err<nom::error::Error<&str>>> {
    let (_, lines) = parse_input(input)?;
    let x_values: Vec<_> = lines.iter().flat_map(|l| l.iter().map(|p| p.x)).collect();
    let max_x = max(500_usize, *x_values.iter().max().unwrap());
    let y_values: Vec<_> = lines.iter().flat_map(|l| l.iter().map(|p| p.y)).collect();
    let max_y = *y_values.iter().max().unwrap();

    let mut array = Array2::from_elem(
        (max_y + PADDING_VALUE, max_x + PADDING_VALUE),
        CaveSpace::Air,
    );

    for line in lines {
        for (current_point, next_point) in line.iter().tuple_windows() {
            let local_min_y = min(current_point.y, next_point.y);
            let local_max_y = max(current_point.y, next_point.y);
            for y in local_min_y..=local_max_y {
                let local_min_x = min(current_point.x, next_point.x);
                let local_max_x = max(current_point.x, next_point.x);
                for x in local_min_x..=local_max_x {
                    array[(y, x)] = CaveSpace::Rock;
                }
            }
        }
    }
    let mut is_sand_stoppable = true;
    let mut current_sand_position = Point { x: 500, y: 0 };
    let mut units_of_sand = 0;
    while is_sand_stoppable {
        match array[(current_sand_position.y + 1, current_sand_position.x)] {
            CaveSpace::Air => {
                current_sand_position.y += 1;
                if current_sand_position.y > max_y {
                    is_sand_stoppable = false;
                }
            }
            CaveSpace::Rock | CaveSpace::Sand => {
                if array[(current_sand_position.y + 1, current_sand_position.x - 1)]
                    == CaveSpace::Air
                {
                    current_sand_position.x -= 1;
                    current_sand_position.y += 1;
                } else if array[(current_sand_position.y + 1, current_sand_position.x + 1)]
                    == CaveSpace::Air
                {
                    current_sand_position.x += 1;
                    current_sand_position.y += 1;
                } else {
                    array[(current_sand_position.y, current_sand_position.x)] = CaveSpace::Sand;
                    current_sand_position = Point::default();
                    units_of_sand += 1;
                }
            }
        }
    }
    Ok(units_of_sand)
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &'static str = include_str!("../../test-input.txt");
    #[test]
    fn part_1_test() {
        assert_eq!(part_1(TEST_INPUT).unwrap(), 24);
    }
}
