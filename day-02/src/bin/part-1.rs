use std::fs;

use day_02::{parse_input_part_1, Move, DRAW_SCORE, WIN_SCORE};

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    if let Ok(score) = part1(&input) {
        println!("Total score is {}", score);
    } else {
        println!("Error parsing the game input");
    }
}

fn part1(input: &str) -> Result<u32, nom::Err<nom::error::Error<&str>>> {
    let (_, games) = parse_input_part_1(input)?;

    let score = games.into_iter().fold(0_u32, |total_score, game| {
        let move_points = game.1.get_score();
        let game_score = match game {
            (Move::Rock, Move::Rock) => DRAW_SCORE,
            (Move::Rock, Move::Paper) => WIN_SCORE,
            (Move::Rock, Move::Scissors) => 0,
            (Move::Paper, Move::Rock) => 0,
            (Move::Paper, Move::Paper) => DRAW_SCORE,
            (Move::Paper, Move::Scissors) => WIN_SCORE,
            (Move::Scissors, Move::Rock) => WIN_SCORE,
            (Move::Scissors, Move::Paper) => 0,
            (Move::Scissors, Move::Scissors) => DRAW_SCORE,
        };
        total_score + move_points + game_score
    });
    Ok(score)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &'static str = include_str!("../../test-input.txt");
    #[test]
    fn part1_test() {
        assert_eq!(part1(TEST_INPUT).unwrap(), 15);
    }
}
