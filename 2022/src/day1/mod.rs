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
    // --- Day 1: Calorie Counting ---
    //
    // Santa's reindeer typically eat regular reindeer food, but they need a lot of magical energy to deliver presents on Christmas.
    // For that, their favorite snack is a special type of star fruit that only grows deep in the jungle.
    // The Elves have brought you on their annual expedition to the grove where the fruit grows.
    //
    // To supply enough magical energy, the expedition needs to retrieve a minimum of fifty stars by December 25th.
    // Although the Elves assure you that the grove has plenty of fruit, you decide to grab any fruit you see along the way, just in case.
    //
    // Collect stars by solving puzzles. Two puzzles will be made available on each day in the Advent calendar;
    // the second puzzle is unlocked when you complete the first. Each puzzle grants one star. Good luck!
    //
    // The jungle must be too overgrown and difficult to navigate in vehicles or access from the air; the Elves' expedition traditionally goes on foot.
    // As your boats approach land, the Elves begin taking inventory of their supplies.
    // One important consideration is food - in particular, the number of Calories each Elf is carrying (your puzzle input).
    //
    // The Elves take turns writing down the number of Calories contained by the various meals, snacks, rations, etc. that they've brought with them, one item per line.
    // Each Elf separates their own inventory from the previous Elf's inventory (if any) by a blank line.
    //
    // For example, suppose the Elves finish writing their items' Calories and end up with the following list:
    //
    // 1000
    // 2000
    // 3000
    //
    // 4000
    //
    // 5000
    // 6000
    //
    // 7000
    // 8000
    // 9000
    //
    // 10000
    //
    // This list represents the Calories of the food carried by five Elves:
    //
    //     The first Elf is carrying food with 1000, 2000, and 3000 Calories, a total of 6000 Calories.
    //     The second Elf is carrying one food item with 4000 Calories.
    //     The third Elf is carrying food with 5000 and 6000 Calories, a total of 11000 Calories.
    //     The fourth Elf is carrying food with 7000, 8000, and 9000 Calories, a total of 24000 Calories.
    //     The fifth Elf is carrying one food item with 10000 Calories.
    //
    // In case the Elves get hungry and need extra snacks, they need to know which Elf to ask:
    // they'd like to know how many Calories are being carried by the Elf carrying the most Calories.
    // In the example above, this is 24000 (carried by the fourth Elf).
    //
    // Find the Elf carrying the most Calories. How many total Calories is that Elf carrying?
    //
    // Your puzzle answer was 64929.
    // --- Part Two ---
    //
    // By the time you calculate the answer to the Elves' question, they've already realized that the Elf carrying the most Calories of food might eventually run out of snacks.
    //
    // To avoid this unacceptable situation, the Elves would instead like to know the total Calories carried by the top three Elves carrying the most Calories.
    // That way, even if one of those Elves runs out of snacks, they still have two backups.
    //
    // In the example above, the top three Elves are the fourth Elf (with 24000 Calories), then the third Elf (with 11000 Calories), then the fifth Elf (with 10000 Calories).
    // The sum of the Calories carried by these three elves is 45000.
    //
    // Find the top three Elves carrying the most Calories. How many Calories are those Elves carrying in total?
    //
    // Your puzzle answer was 193697.
    puzzle_1();
    puzzle_2();
}
