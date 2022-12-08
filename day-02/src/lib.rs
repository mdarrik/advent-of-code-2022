use nom::{
    branch::alt,
    character::complete::{char, newline, space1},
    multi::separated_list0,
    sequence::separated_pair,
    IResult,
};

pub const WIN_SCORE: u32 = 6;
pub const DRAW_SCORE: u32 = 3;

pub fn parse_input_part_1(input: &str) -> IResult<&str, Vec<(Move, Move)>> {
    separated_list0(newline, game)(input)

    // Ok(vec![])
}

fn opponent_rock(input: &str) -> IResult<&str, Move> {
    let (input, _) = nom::character::complete::char('A')(input)?;
    Ok((input, Move::Rock))
}

fn opponent_paper(input: &str) -> IResult<&str, Move> {
    let (input, _) = char('B')(input)?;
    Ok((input, Move::Paper))
}

fn opponent_scissors(input: &str) -> IResult<&str, Move> {
    let (input, _) = char('C')(input)?;
    Ok((input, Move::Scissors))
}

fn player_rock(input: &str) -> IResult<&str, Move> {
    let (input, _) = char('X')(input)?;
    Ok((input, Move::Rock))
}

fn player_paper(input: &str) -> IResult<&str, Move> {
    let (input, _) = char('Y')(input)?;
    Ok((input, Move::Paper))
}

fn player_scissors(input: &str) -> IResult<&str, Move> {
    let (input, _) = char('Z')(input)?;
    Ok((input, Move::Scissors))
}

fn game(input: &str) -> IResult<&str, (Move, Move)> {
    separated_pair(
        opponent_move,
        space1,
        alt((player_rock, player_paper, player_scissors)),
    )(input)
}

fn opponent_move(input: &str) -> IResult<&str, Move> {
    alt((opponent_rock, opponent_paper, opponent_scissors))(input)
}

pub fn parse_input_part_2(input: &str) -> IResult<&str, Vec<(Move, GameResult)>> {
    separated_list0(newline, expected_game)(input)
}

fn expected_game(input: &str) -> IResult<&str, (Move, GameResult)> {
    separated_pair(
        opponent_move,
        space1,
        alt((result_lose, result_win, result_draw)),
    )(input)
}

fn result_lose(input: &str) -> IResult<&str, GameResult> {
    let (input, _) = char('X')(input)?;
    Ok((input, GameResult::Lose))
}

fn result_draw(input: &str) -> IResult<&str, GameResult> {
    let (input, _) = char('Y')(input)?;
    Ok((input, GameResult::Draw))
}

fn result_win(input: &str) -> IResult<&str, GameResult> {
    let (input, _) = char('Z')(input)?;
    Ok((input, GameResult::Win))
}

pub enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    pub fn get_score(&self) -> u32 {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }
}

pub enum GameResult {
    Win,
    Lose,
    Draw,
}

impl GameResult {
    pub fn get_move(&self, game_move: &Move) -> Move {
        match (self, game_move) {
            (GameResult::Win, Move::Rock) => Move::Paper,
            (GameResult::Win, Move::Paper) => Move::Scissors,
            (GameResult::Win, Move::Scissors) => Move::Rock,
            (GameResult::Lose, Move::Rock) => Move::Scissors,
            (GameResult::Lose, Move::Paper) => Move::Rock,
            (GameResult::Lose, Move::Scissors) => Move::Paper,
            (GameResult::Draw, Move::Rock) => Move::Rock,
            (GameResult::Draw, Move::Paper) => Move::Paper,
            (GameResult::Draw, Move::Scissors) => Move::Scissors,
        }
    }
    pub fn get_score(&self, game_move: &Move) -> u32 {
        let move_choice = self.get_move(game_move);
        let result_score = match self {
            GameResult::Win => WIN_SCORE,
            GameResult::Lose => 0,
            GameResult::Draw => DRAW_SCORE,
        };

        move_choice.get_score() + result_score
    }
}
