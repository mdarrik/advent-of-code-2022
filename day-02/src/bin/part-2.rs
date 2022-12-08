use std::fs;

use day_02::parse_input_part_2;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    if let Ok(score) = part2(&input) {
        println!("Total score is {}", score);
    } else {
        println!("Error parsing the game input");
    }
}

fn part2(input: &str) -> Result<u32, nom::Err<nom::error::Error<&str>>> {
    let (_, games) = parse_input_part_2(input)?;

    let score = games
        .iter()
        .fold(0_u32, |total_score, (opponent_move, game_result)| {
            let score = game_result.get_score(opponent_move);
            total_score + score
        });
    Ok(score)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &'static str = include_str!("../../test-input.txt");
    #[test]
    fn part2_test() {
        assert_eq!(part2(TEST_INPUT).unwrap(), 12);
    }
}
