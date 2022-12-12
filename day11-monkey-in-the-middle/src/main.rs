use std::collections::VecDeque;

use regex::Regex;

fn main() {
    // Read in the monkey notes from the question trimming any surrounding whitespace
    let question_monkey_notes = include_str!("../inputs/question").trim();

    // * Part One
    // Get the level of monkey business based on the number of items inspected by monkeys in the input from the question
    let total_monkey_business = part_one(question_monkey_notes);
    println!("Part One:\n  Total item inspection monkey business is: {total_monkey_business}");

    // * Part Two
    // <Whatever part two is doing>
    let part_two_result = part_two(question_monkey_notes);
    println!("Part Two:\n  The result is: {part_two_result}");
}

// region: Helpers

fn parse_monkey_notes(monkey_notes: &str) -> Vec<Monkey> {
    monkey_notes
        .split("\n\n")
        .map(|monkey_section| {
            // Construct a regex to parse the relevant info from the monkey section
            let re = Regex::new(
                    "Monkey (?P<monkey_num>[0-9]+):\n\
                    [ ]*Starting items: (?P<item_list>.*)\n\
                    [ ]*Operation: new = old (?P<operation>.*)\n\
                    [ ]*Test: divisible by (?P<divisor_test>[0-9]+)\n\
                    [ ]*If true: throw to monkey (?P<true_monkey>[0-9]+)\n\
                    [ ]*If false: throw to monkey (?P<false_monkey>[0-9]+)"
                )
                .expect("Bad regex defined");

            // Perform the regex match on the monkey_section
            let caps = re.captures(monkey_section).unwrap();

            // Get the monkey number
            let monkey_num = caps.name("monkey_num").unwrap().as_str();
            let monkey_num = monkey_num.parse::<usize>().expect("Bad monkey number");
            // println!("Monkey num: {monkey_num}", );

            // Get the list of items the monkey is starting with
            let item_list = caps.name("item_list").unwrap().as_str();
            let item_list = item_list
                .split(", ")
                .map(|num| num.parse::<u32>().unwrap())
                .collect::<VecDeque<u32>>();
            // println!("Item list: {item_list}", );

            // Get the operation the monkey does on inspection
            let operation = caps.name("operation").unwrap().as_str();
            // let operator = get_operator_func(operation.to_string());
            // println!("Operation: {operation}", );

            // Get the divisor used by the monkey for its test
            let divisor_test = caps.name("divisor_test").unwrap().as_str();
            let divisor_test = divisor_test.parse::<u32>().expect("Bad divisor test number");
            // println!("Disible by: {divisor_test}", );

            // Get the number of the monkey to throw an item to if the test result is true
            let true_monkey = caps.name("true_monkey").unwrap().as_str();
            let true_monkey = true_monkey.parse::<usize>().expect("Bad monkey number");
            // println!("Monkey if true: {true_monkey}", );

            // Get the number of the monkey to throw an item to if the test result is false
            let false_monkey = caps.name("false_monkey").unwrap().as_str();
            let false_monkey = false_monkey.parse::<usize>().expect("Bad monkey number");
            // println!("Monkey if false: {false_monkey}", );
            
            // Return the monkey info
            Monkey {
                number: monkey_num,
                starting_items: item_list,
                operation: operation.to_string(),
                divisor_test,
                true_monkey,
                false_monkey,
            }
        })
        .collect::<Vec<Monkey>>()
}

// fn get_operator_func(operation: String) -> Box<dyn Fn(u32) -> u32> {
//     // 
//     match operation.split_once(" ").expect("Bad operation definition") {
//         // Operators can only be + or *
//         ("+", "old") => Box::new(|old: u32| old + old),
//         ("*", "old") => Box::new(|old: u32| old * old),
//         ("+", num) => Box::new(|old: u32| old + num.parse::<u32>().unwrap().clone()),
//         ("*", num) => Box::new(|old: u32| old * num.parse::<u32>().unwrap()),
//         _ => panic!("Invalid operation format")
//     }
// }

struct Monkey {
    number: usize,
    starting_items: VecDeque<u32>,
    operation: String,
    divisor_test: u32,
    true_monkey: usize,
    false_monkey: usize,
}

// endregion

// region: Part One

#[test]
fn part_one_example_test() {
    // Read in the example monkey notes trimming any surrounding whitespace
    let example_monkey_notes = include_str!("../inputs/example").trim();

    // Get the level of monkey business based on the number of items inspected by monkeys in the example input from the question
    let total_monkey_business = part_one(example_monkey_notes);

    // Check if the example yields the same result as the question describes
    assert_eq!(total_monkey_business, 10605);
}

/// Calculate the total amount of monkey business that happens over the course of 20 rounds
fn part_one(monkey_notes: &str) -> u32 {
    // Parse the notes about each monkey into a vector of Monkey structs
    let monkey_notes = parse_monkey_notes(monkey_notes);

    // Create a mutable list of each monkey's items that we can use to keep track of which they're holding throughout the rounds
    let mut monkey_items = monkey_notes
        .iter()
        .map(|monkey| monkey.starting_items.clone())
        .collect::<Vec<_>>();

    let mut inspect_counts = vec![0u32; monkey_notes.len()];

    // Play out 20 rounds of monkey business
    for _ in 0..20 {
        // Go through each monkey's turn
        for (monkey_num, monkey) in monkey_notes.iter().enumerate() {
            // println!("Monkey {monkey_num}:");
            // For each element in their inventory...
            while let Some(mut worry_level) = monkey_items.get_mut(monkey_num).unwrap().pop_front() {
                // println!("  Monkey inspects an item with a worry level of {worry_level}.");
                // Inspect the item - apply the operation, modifying the worry_level accordingly
                worry_level = match monkey.operation.split_once(" ").expect("Bad operation definition") {
                    // Operators can only be + or *
                    ("+", "old") => worry_level + worry_level,
                    ("*", "old") => worry_level * worry_level,
                    ("+", num) => worry_level + num.parse::<u32>().unwrap().clone(),
                    ("*", num) => worry_level * num.parse::<u32>().unwrap(),
                    _ => panic!("Invalid operation format")
                };
                // println!("    After operation {}, worry level becomes {worry_level}", monkey.operation);

                // Record that an item was inspected by this monkey
                *inspect_counts.get_mut(monkey_num).unwrap() += 1;

                // Perform relief division on worry_level seeing that the item wasn't broken
                worry_level /= 3;
                // println!("    Monkey gets bored with item. Worry level is divided by 3 to {worry_level}.");

                // Perform monkey test and send the item to the appropriate monkey depending on the result
                let dest_monkey = if worry_level % monkey.divisor_test == 0 {
                    monkey.true_monkey
                }
                else {
                    monkey.false_monkey
                };
                // println!("    Item with worry level {worry_level} is thrown to monkey {dest_monkey}.");
                monkey_items.get_mut(dest_monkey).unwrap().push_back(worry_level);
            }
        }
    }

    // Find the two most inspective monkeys over the 20 rounds and multiply their inspectiveness for the total monkey business
    println!("Inspect counts: {:?}", inspect_counts);
    inspect_counts.sort();
    inspect_counts.iter().rev().cloned().take(2).product()
}

// endregion

// region: Part Two

#[test]
fn part_two_example_test() {
    // Read in the example monkey notes trimming any surrounding whitespace
    let example_monkey_notes = include_str!("../inputs/example").trim();

    // <Part Two goal> in the example input from the question
    let part_two_result = part_two(example_monkey_notes);

    // Check if the example yields the same result as the question describes
    assert_eq!(part_two_result, 0);
}

/// <Part Two goal
fn part_two(monkey_notes: &str) -> u32 {
    // <Describe what parsing the input is doing>
    let parsed_monkey_notes = parse_monkey_notes(monkey_notes);

    // The rest...
    0
}

// endregion
