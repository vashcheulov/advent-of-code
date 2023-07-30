use crate::day2::utils::{calculate_score, read_input, Round, RoundResult, Shape};

fn parse_round(opponent: &str, me: &str) -> Round {
    let opponent_shape = match opponent {
        "A" => Shape::Rock,
        "B" => Shape::Paper,
        "C" => Shape::Scissors,
        _ => panic!("Unknown shape"),
    };
    let my_shape = match me {
        "X" => Shape::Rock,
        "Y" => Shape::Paper,
        "Z" => Shape::Scissors,
        _ => panic!("Unknown shape"),
    };
    return Round::new(opponent_shape, my_shape, RoundResult::Win);
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

pub fn puzzle_1() {
    let input = read_input();
    let rounds: Vec<Round> = parse_input(input);
    let score = calculate_score(rounds);
    println!("Score: {}", score);
    // Score: 15337
}
