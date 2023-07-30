use crate::day2::utils::{calculate_score, read_input, Round, RoundResult, Shape};

fn parse_round(opponent: &str, result: &str) -> Round {
    let opponent_shape = match opponent {
        "A" => Shape::Rock,
        "B" => Shape::Paper,
        "C" => Shape::Scissors,
        _ => panic!("Unknown shape"),
    };
    let result = match result {
        "X" => RoundResult::Loss,
        "Y" => RoundResult::Draw,
        "Z" => RoundResult::Win,
        _ => panic!("Unknown shape"),
    };
    let my_shape = match result {
        RoundResult::Win => match opponent_shape {
            Shape::Rock => Shape::Paper,
            Shape::Paper => Shape::Scissors,
            Shape::Scissors => Shape::Rock,
        },
        RoundResult::Draw => opponent_shape.clone(),
        RoundResult::Loss => match opponent_shape {
            Shape::Rock => Shape::Scissors,
            Shape::Paper => Shape::Rock,
            Shape::Scissors => Shape::Paper,
        },
    };
    return Round::new(opponent_shape, my_shape, result);
}

fn parse_input(input: String) -> Vec<Round> {
    let mut rounds: Vec<Round> = Vec::new();
    for line in input.lines() {
        let round_tuple: Vec<&str> = line.split(" ").collect();
        let round = parse_round(round_tuple[0], round_tuple[1]);
        rounds.push(round);
    }
    rounds
}

pub fn puzzle_2() {
    let input = read_input();
    let rounds: Vec<Round> = parse_input(input);
    let score = calculate_score(rounds);
    println!("Score: {}", score);
    // Score: 11696
}
