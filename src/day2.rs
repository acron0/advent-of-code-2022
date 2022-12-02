use crate::util;

#[derive(Copy, Clone)]
enum Move {
    Rock = 1,
    Paper = 2,
    Scissors = 3
}

#[derive(Copy, Clone)]
enum Outcome {
    PlayerWins = 6,
    OpponentWins = 0,
    Draw = 3
}

fn decide (player: &Move, opponent: &Move) -> Outcome {
    match [player, opponent] {
        [Move::Rock,     Move::Paper]    => Outcome::OpponentWins,
        [Move::Rock,     Move::Scissors] => Outcome::PlayerWins,
        [Move::Paper,    Move::Rock]     => Outcome::PlayerWins,
        [Move::Paper,    Move::Scissors] => Outcome::OpponentWins,
        [Move::Scissors, Move::Rock]     => Outcome::OpponentWins,
        [Move::Scissors, Move::Paper]    => Outcome::PlayerWins,
        //
        _ => Outcome::Draw
    }
}

fn code_to_move(code: char) -> Move {
    match code {
        'A' => Move::Rock,
        'B' => Move::Paper,
        'C' => Move::Scissors,
        //
        'X' => Move::Rock,
        'Y' => Move::Paper,
        'Z' => Move::Scissors,
        //
        _ => todo!()
    }
}

pub fn run1() -> u32 {
    if let Ok(lines) = util::read_lines("./inputs/day2") {
        let mut accumulated_score: u32 = 0;
        for line in lines {
            if let Ok(strategy) = line {
                let chars: Vec<_> = strategy.chars().collect();
                if chars.len() >= 3 {
                    let opponent_move = code_to_move(chars[0]);
                    let player_move   = code_to_move(chars[2]);
                    let outcome       = decide(&player_move, &opponent_move);
                    let score         = player_move as u32 + outcome as u32;

                    accumulated_score += score;
                    //println!("{} -> {} {}", strategy, score, accumulated_score)

                }
            }
        }
        return accumulated_score
    } else {
        return 0
    }
}
pub const EXPECTED_RESULT1: u32 = 17189;

////////////////////////////////////////////////////////////////////////////////

fn code_to_outcome(code: char) -> Outcome {
    match code {
        'X' => Outcome::OpponentWins,
        'Y' => Outcome::Draw,
        'Z' => Outcome::PlayerWins,
        //
        _ => todo!()
    }
}

fn winning_move(mmove: &Move) -> Move {
    match mmove {
        Move::Rock     => Move::Paper,
        Move::Paper    => Move::Scissors,
        Move::Scissors => Move::Rock
    }
}

fn losing_move(mmove: &Move) -> Move {
    match mmove {
        Move::Rock     => Move::Scissors,
        Move::Paper    => Move::Rock,
        Move::Scissors => Move::Paper
    }
}

fn cheat(opponent_move: &Move, outcome: &Outcome) -> Move {
    match outcome {
        Outcome::OpponentWins => return losing_move(opponent_move),
        Outcome::PlayerWins   => return winning_move(opponent_move),
        Outcome::Draw         => return *opponent_move
    }
}

pub fn run2() -> u32 {
    if let Ok(lines) = util::read_lines("./inputs/day2") {
        let mut accumulated_score: u32 = 0;
        for line in lines {
            if let Ok(strategy) = line {
                let chars: Vec<_> = strategy.chars().collect();
                if chars.len() >= 3 {
                    let opponent_move = code_to_move(chars[0]);
                    let outcome       = code_to_outcome(chars[2]);
                    let player_move   = cheat(&opponent_move, &outcome);
                    let score         = player_move as u32 + outcome as u32;

                    accumulated_score += score;
                    //println!("{} -> {} {}", strategy, score, accumulated_score)

                }
            }
        }
        return accumulated_score
    } else {
        return 0
    }
}
pub const EXPECTED_RESULT2: u32 = 13490;
