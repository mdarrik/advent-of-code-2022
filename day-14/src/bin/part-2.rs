use std::{
    cmp::{max, min},
    fs,
};

use day_14::{parse_input, CaveSpace, Point};
use itertools::Itertools;
use ndarray::Array2;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let units_of_sand = part_2(&input).unwrap();
    println!("units of sand {}", units_of_sand);
}

fn part_2(input: &str) -> Result<u32, nom::Err<nom::error::Error<&str>>> {
    let (_, lines) = parse_input(input)?;
    let x_values: Vec<_> = lines.iter().flat_map(|l| l.iter().map(|p| p.x)).collect();
    let max_x = max(500_usize, *x_values.iter().max().unwrap());
    let y_values: Vec<_> = lines.iter().flat_map(|l| l.iter().map(|p| p.y)).collect();
    let max_y = *y_values.iter().max().unwrap();

    let mut array = Array2::from_elem(
        (max_y + 3, max_x + 100),
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
    let mut current_sand_position = Point { x: 500, y: 0 };
    let mut units_of_sand = 0;
    while array[(0, 500)] == CaveSpace::Air {
        let next_sand_position = Point {y: current_sand_position.y + 1, x: current_sand_position.x};
        if next_sand_position.y == max_y + 2 {
            array[(current_sand_position.y, current_sand_position.x)] = CaveSpace::Sand;
            units_of_sand += 1;
            current_sand_position = Point::default();
            continue;
        }
        match array[(next_sand_position.y, next_sand_position.x)] {
            CaveSpace::Air => {
                current_sand_position.y += 1;
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
    fn part_2_test() {
        assert_eq!(part_2(TEST_INPUT).unwrap(), 93);
    }
}
