use std::fs;

#[derive(Debug, Clone)]
pub enum Shape {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug)]
pub enum RoundResult {
    Win,
    Draw,
    Loss,
}

#[derive(Debug)]
pub struct Round {
    pub opponent_shape: Shape,
    pub my_shape: Shape,
    pub result: RoundResult,
}

impl Round {
    pub fn new(opponent_shape: Shape, my_shape: Shape, result: RoundResult) -> Round {
        Round {
            opponent_shape,
            my_shape,
            result,
        }
    }
}

pub fn read_input() -> String {
    return fs::read_to_string("src/day2/input.txt")
        .expect("Something went wrong reading the file");
}

pub fn calculate_score(rounds: Vec<Round>) -> i32 {
    let mut score = 0;
    for round in rounds {
        let round_score = match round.opponent_shape {
            Shape::Rock => match round.my_shape {
                Shape::Rock => 3,
                Shape::Paper => 6,
                Shape::Scissors => 0,
            },
            Shape::Paper => match round.my_shape {
                Shape::Rock => 0,
                Shape::Paper => 3,
                Shape::Scissors => 6,
            },
            Shape::Scissors => match round.my_shape {
                Shape::Rock => 6,
                Shape::Paper => 0,
                Shape::Scissors => 3,
            },
        };
        let shape_score = match round.my_shape {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        };
        score += round_score + shape_score;
    }
    score
}
