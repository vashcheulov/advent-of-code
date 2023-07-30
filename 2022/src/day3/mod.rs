use std::fs;

const PRIORITIES: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

#[derive(Debug, Clone)]
struct Rucksack {
    first_compartment: String,
    second_compartment: String,
}

impl Rucksack {
    fn new(first_compartment: String, second_compartment: String) -> Rucksack {
        Rucksack {
            first_compartment,
            second_compartment,
        }
    }

    fn to_string(&self) -> String {
        format!("{}{}", self.first_compartment, self.second_compartment)
    }

    fn compartment_intersection(&self) -> Option<char> {
        for c in self.first_compartment.chars() {
            if self.second_compartment.contains(c) {
                return Some(c);
            }
        }
        return None;
    }
}

#[derive(Debug)]
struct RucksackGroup {
    rucksacks: (Rucksack, Rucksack, Rucksack),
}

impl RucksackGroup {
    fn new(rucksacks: (Rucksack, Rucksack, Rucksack)) -> RucksackGroup {
        RucksackGroup { rucksacks }
    }

    fn badge_intersection(&self) -> Option<char> {
        for c in self.rucksacks.0.to_string().chars() {
            if self.rucksacks.1.to_string().contains(c) && self.rucksacks.2.to_string().contains(c)
            {
                return Some(c);
            }
        }
        return None;
    }
}

pub fn read_input() -> String {
    return fs::read_to_string("src/day3/input.txt")
        .expect("Something went wrong reading the file");
}

fn parse_input(input: String) -> Vec<Rucksack> {
    let mut rucksacks: Vec<Rucksack> = Vec::new();
    for line in input.lines() {
        let (first_compartment, second_compartment) = line.split_at(line.len() / 2);
        let rucksack = Rucksack::new(
            first_compartment.to_string(),
            second_compartment.to_string(),
        );
        rucksacks.push(rucksack);
    }
    rucksacks
}

fn calculate_total_rucksacks_score(rucksacks: Vec<Rucksack>) -> i32 {
    let mut score = 0;
    for rucksack in rucksacks {
        score = match rucksack.compartment_intersection() {
            Some(c) => score + (PRIORITIES.find(c).unwrap() as i32 + 1),
            None => score,
        };
    }
    score
}

fn calculate_total_group_score(rucksack_groups: Vec<RucksackGroup>) -> i32 {
    let mut score = 0;
    for rucksack_group in rucksack_groups {
        score = match rucksack_group.badge_intersection() {
            Some(c) => score + (PRIORITIES.find(c).unwrap() as i32 + 1),
            None => score,
        };
    }
    score
}

fn create_rucksack_groups(rucksacks: Vec<Rucksack>) -> Vec<RucksackGroup> {
    let mut rucksack_groups: Vec<RucksackGroup> = Vec::new();
    for rucksacks in rucksacks.chunks(3) {
        rucksack_groups.push(RucksackGroup::new({
            (
                rucksacks[0].clone(),
                rucksacks[1].clone(),
                rucksacks[2].clone(),
            )
        }));
    }

    rucksack_groups
}

fn puzzle_1() {
    let input = read_input();
    let rucksacks: Vec<Rucksack> = parse_input(input);
    let score = calculate_total_rucksacks_score(rucksacks);
    println!("Puzzle 1 score: {}", score);
    // Puzzle 1 score: 7831
}

fn puzzle_2() {
    let input = read_input();
    let rucksacks: Vec<Rucksack> = parse_input(input);
    let rucksack_groups = create_rucksack_groups(rucksacks);
    let score = calculate_total_group_score(rucksack_groups);
    println!("Puzzle 2 score: {}", score);
    // Puzzle 2 score: 2683
}

pub(crate) fn run() {
    // --- Day 3: Rucksack Reorganization ---
    //
    // One Elf has the important job of loading all of the rucksacks with supplies for the jungle journey.
    // Unfortunately, that Elf didn't quite follow the packing instructions, and so a few items now need to be rearranged.
    //
    // Each rucksack has two large compartments. All items of a given type are meant to go into exactly one of the two compartments.
    // The Elf that did the packing failed to follow this rule for exactly one item type per rucksack.
    //
    // The Elves have made a list of all of the items currently in each rucksack (your puzzle input), but they need your help finding the errors.
    // Every item type is identified by a single lowercase or uppercase letter (that is, a and A refer to different types of items).
    //
    // The list of items for each rucksack is given as characters all on a single line.
    // A given rucksack always has the same number of items in each of its two compartments, so the first half of the characters represent items in the first compartment,
    // while the second half of the characters represent items in the second compartment.
    //
    // For example, suppose you have the following list of contents from six rucksacks:
    //
    // vJrwpWtwJgWrhcsFMMfFFhFp
    // jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
    // PmmdzqPrVvPwwTWBwg
    // wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
    // ttgJtRGJQctTZtZT
    // CrZsJsPPZsGzwwsLwLmpwMDw
    //
    //     The first rucksack contains the items vJrwpWtwJgWrhcsFMMfFFhFp, which means its first compartment contains the items vJrwpWtwJgWr,
    //     while the second compartment contains the items hcsFMMfFFhFp. The only item type that appears in both compartments is lowercase p.
    //     The second rucksack's compartments contain jqHRNqRjqzjGDLGL and rsFMfFZSrLrFZsSL. The only item type that appears in both compartments is uppercase L.
    //     The third rucksack's compartments contain PmmdzqPrV and vPwwTWBwg; the only common item type is uppercase P.
    //     The fourth rucksack's compartments only share item type v.
    //     The fifth rucksack's compartments only share item type t.
    //     The sixth rucksack's compartments only share item type s.
    //
    // To help prioritize item rearrangement, every item type can be converted to a priority:
    //
    //     Lowercase item types a through z have priorities 1 through 26.
    //     Uppercase item types A through Z have priorities 27 through 52.
    //
    // In the above example, the priority of the item type that appears in both compartments of each rucksack is 16 (p), 38 (L), 42 (P), 22 (v), 20 (t), and 19 (s); the sum of these is 157.
    //
    // Find the item type that appears in both compartments of each rucksack. What is the sum of the priorities of those item types?
    //
    // --- Part Two ---
    //
    // As you finish identifying the misplaced items, the Elves come to you with another issue.
    //
    // For safety, the Elves are divided into groups of three. Every Elf carries a badge that identifies their group.
    // For efficiency, within each group of three Elves, the badge is the only item type carried by all three Elves.
    // That is, if a group's badge is item type B, then all three Elves will have item type B somewhere in their rucksack, and at most two of the Elves will be carrying any other item type.
    //
    // The problem is that someone forgot to put this year's updated authenticity sticker on the badges.
    // All of the badges need to be pulled out of the rucksacks so the new authenticity stickers can be attached.
    //
    // Additionally, nobody wrote down which item type corresponds to each group's badges.
    // The only way to tell which item type is the right one is by finding the one item type that is common between all three Elves in each group.
    //
    // Every set of three lines in your list corresponds to a single group, but each group can have a different badge item type.
    // So, in the above example, the first group's rucksacks are the first three lines:
    //
    // vJrwpWtwJgWrhcsFMMfFFhFp
    // jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
    // PmmdzqPrVvPwwTWBwg
    //
    // And the second group's rucksacks are the next three lines:
    //
    // wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
    // ttgJtRGJQctTZtZT
    // CrZsJsPPZsGzwwsLwLmpwMDw
    //
    // In the first group, the only item type that appears in all three rucksacks is lowercase r; this must be their badges.
    // In the second group, their badge item type must be Z.
    //
    // Priorities for these items must still be found to organize the sticker attachment efforts:
    // here, they are 18 (r) for the first group and 52 (Z) for the second group. The sum of these is 70.
    //
    // Find the item type that corresponds to the badges of each three-Elf group. What is the sum of the priorities of those item types?
    //
    puzzle_1();
    puzzle_2();
}
