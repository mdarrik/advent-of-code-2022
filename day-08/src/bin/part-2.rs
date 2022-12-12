use std::fs;

use day_08::parse_input;
use ndarray::Array2;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let num_visible = part_2(&input).unwrap();
    println!("tree with best view has a score of {}", num_visible);
}

fn part_2(input: &str) -> Result<u32, nom::Err<nom::error::Error<&str>>> {
    let (_, trees) = parse_input(input)?;
    let (num_rows, num_cols) = trees.dim();
    let mut tree_scores: Array2<u32> = Array2::zeros((num_rows, num_cols));

    let mut max_score = 0;

    for ((row, col), height) in trees.indexed_iter() {
        if row == 0 || col == 0 || row == num_rows - 1 || col == num_cols - 1 {
            // if it's an edge, the view score is 0 so just skip this iteration
            continue;
        }
        let mut up_score = 0_u32;
        let mut down_score = 0_u32;
        let mut right_score = 0_u32;
        let mut left_score = 0_u32;

        for new_row in (row + 1)..num_rows {
            if let Some(compare_height) = trees.get((new_row, col)) {
                down_score += 1;
                if compare_height >= height {
                    break;
                }
            }
        }

        for new_row in (0..row).rev() {
            if let Some(compare_height) = trees.get((new_row, col)) {
                up_score += 1;
                if compare_height >= height {
                    break;
                }
            }
        }

        for new_col in (col + 1)..num_cols {
            if let Some(compare_height) = trees.get((row, new_col)) {
                right_score += 1;
                if compare_height >= height {
                    break;
                }
            }
        }

        for new_col in (0..col).rev() {
            if let Some(compare_height) = trees.get((row, new_col)) {
                left_score += 1;
                if compare_height >= height {
                    break;
                }
            }
        }
        let current_score = up_score * down_score * left_score * right_score;
        tree_scores[(row, col)] = current_score;
        if current_score > max_score {
            max_score = current_score;
        }
    }

    Ok(max_score)
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &'static str = include_str!("../../test-input.txt");
    #[test]
    fn part_2_test() {
        assert_eq!(part_2(TEST_INPUT).unwrap(), 8);
    }
}
