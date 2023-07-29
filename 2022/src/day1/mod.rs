use std::{collections::HashMap, fs};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Elve {
    index: i32,
    calories_entries: Vec<i32>,
}

impl Elve {
    fn new(index: i32) -> Elve {
        Elve {
            index,
            calories_entries: Vec::new(),
        }
    }

    fn add_calories(&mut self, calories: i32) {
        self.calories_entries.push(calories);
    }

    fn get_calories(&self) -> i32 {
        let mut sum = 0;
        for calories in &self.calories_entries {
            sum += calories;
        }
        sum
    }
}

fn parse_input() -> Vec<Elve> {
    // read file
    let content =
        fs::read_to_string("src/day1/input.txt").expect("Something went wrong reading the file");

    let mut elves: Vec<Elve> = Vec::new();

    let mut elve_count = 1;
    let mut current_elve = Elve::new(elve_count);
    for line in content.lines() {
        if line != "" {
            current_elve.add_calories(line.parse::<i32>().unwrap());
        } else {
            elves.push(current_elve.clone());
            elve_count += 1;
            current_elve = Elve::new(elve_count);
        }
    }
    elves.push(current_elve.clone());

    elves
}

fn puzzle_1() {
    let elves = parse_input();
    let elve_with_most_calories = elves.iter().max_by_key(|elve| elve.get_calories()).unwrap();

    println!(
        "Elve with most calories: {:?}, total_calories: {:?}",
        elve_with_most_calories,
        elve_with_most_calories.get_calories()
    );
    // Elve with most calories: Elve { index: 127, calories_entries: [1596, 5306, 5192, 7398, 7306, 7457, 2157, 4592, 7133, 5658, 6134, 5000] }, total_calories: 64929
}

fn puzzle_2() {
    let elves = parse_input();
    let mut top_elves: Vec<Elve> = Vec::new();
    while top_elves.len() < 3 {
        let elve_with_most_calories = elves
            .iter()
            .filter(|elve| !top_elves.contains(elve))
            .max_by_key(|elve| elve.get_calories())
            .unwrap();
        top_elves.push(elve_with_most_calories.clone());
    }
    let top_elves_total_calories = top_elves
        .iter()
        .fold(0, |acc, elve| acc + elve.get_calories());
    println!(
        "Top 3 elves with most calories: {:?}, total_calories: {:?}",
        top_elves, top_elves_total_calories
    );
    // Elves with most calories: [Elve { index: 127, calories_entries: [1596, 5306, 5192, 7398, 7306, 7457, 2157, 4592, 7133, 5658, 6134, 5000] }, Elve { index: 112, calories_entries: [36692, 27998] }, Elve { index: 203, calories_entries: [64078] }], total_calories: 193697
}

pub(crate) fn run() {
    puzzle_1();
    puzzle_2();
}
