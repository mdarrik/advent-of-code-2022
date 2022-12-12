use std::fs;

use day_08::parse_input;
use ndarray::Array2;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let num_visible = part_1(&input).unwrap();
    println!("number of visible trees is {}", num_visible);
}

fn part_1(input: &str) -> Result<u16, nom::Err<nom::error::Error<&str>>> {
    let (_, trees) = parse_input(input)?;
    let (num_rows, num_cols) = trees.dim();
    dbg!(num_rows, num_cols);
    let mut visible_tree: Array2<u8> = Array2::zeros((num_rows, num_cols));
    // visible_tree.row_mut(0).map_inplace(|s| *s = Some(true));
    // visible_tree.row_mut(num_rows -1).map_inplace(|s| *s = Some(true));
    // visible_tree.column_mut(0).map_inplace(|s| *s = Some(true));
    // visible_tree.column_mut(num_cols -1).map_inplace(|s| *s = Some(true));
    for row in 0..num_rows {
        let mut row_max = (row, 0);
        let mut col_max = (0, row);
        for col in 0..num_cols {
            let current_height = trees.get((row, col)).unwrap();
            if let Some(previous_height) = trees.get((row, col.wrapping_sub(1))) {
                if previous_height < current_height && *current_height > trees[row_max] {
                    visible_tree[(row, col)] = 1;
                    row_max = (row, col);
                }
            } else {
                visible_tree[(row, col)] = 1;
                row_max = (row, col);
            }
            let transposed_height = trees.get((col, row)).unwrap();
            if let Some(previous_height) = trees.get((col.wrapping_sub(1), row)) {
                if previous_height < transposed_height && *transposed_height > trees[col_max] {
                    visible_tree[(col, row)] = 1;
                    col_max = (col, row);
                }
            } else {
                visible_tree[(col, row)] = 1;
                col_max = (col, row);
            }
        }
        let mut row_max = (row, num_cols - 1);
        let mut col_max = (num_cols - 1, row);
        for col in (0..num_cols).rev() {
            let current_height = trees.get((row, col)).unwrap();
            if let Some(previous_height) = trees.get((row, col + 1)) {
                if previous_height < current_height && *current_height > trees[row_max] {
                    visible_tree[(row, col)] = 1;
                    row_max = (row, col);
                }
            } else {
                visible_tree[(row, col)] = 1;
                row_max = (row, col);
            }
            let transposed_height = trees.get((col, row)).unwrap();
            if let Some(previous_height) = trees.get((col + 1, row)) {
                if previous_height < transposed_height && *transposed_height > trees[col_max] {
                    visible_tree[(col, row)] = 1;
                    col_max = (col, row)
                }
            } else {
                visible_tree[(col, row)] = 1;
                col_max = (col, row)
            }
        }
    }
    let num_visible = visible_tree.fold(0, |count_visible, el| {
        if el == &1 {
            count_visible + 1
        } else {
            count_visible
        }
    });
    Ok(num_visible)
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &'static str = include_str!("../../test-input.txt");
    #[test]
    fn part_1_test() {
        assert_eq!(part_1(TEST_INPUT).unwrap(), 21);
    }
}
