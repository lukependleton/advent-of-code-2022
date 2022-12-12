use std::collections::{VecDeque, HashMap};

use regex::Regex;

fn main() {
    // Read in the monkey notes from the question trimming any surrounding whitespace
    let question_monkey_notes = include_str!("../inputs/question").trim();

    // * Part One
    // Get the level of monkey business based on the number of items inspected by monkeys in the input from the question
    let total_monkey_business = part_one(question_monkey_notes);
    println!("Part One:\n  Total item inspection monkey business over 20 rounds is: {total_monkey_business}");

    // * Part Two
    // Get the total monkey business over 10000 rounds of the question's monkey input using modular arithmetic to manage large numbers
    let part_two_result = part_two(question_monkey_notes);
    println!("Part Two:\n  Total item inspection monkey business over 10000 rounds is: {part_two_result}");
}

// region: Helpers

struct Monkey {
    starting_items: VecDeque<u64>,
    operation: Operation,
    divisor_test: u64,
    true_monkey: usize,
    false_monkey: usize,
}

#[derive(Clone, Debug)]
enum Operation {
    Add(u64),
    Multiply(u64),
    // Would make more sense to to exponent, but this program only ever squares
    Square(),
}

#[derive(Clone, Debug)]
enum ItemWorryRepresentation {
    Value(u64),
    OpertionChain {
        base_worry: u64,
        operation_chain: Vec<Operation>,
        modulo_cache: HashMap<u64, (usize, u64)>,
    }
}

/// Regex parse the monkey notes string into a vector of Monkey objects
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

            // Don't really need to get the monkey number - the index of the monkey is this number

            // Get the list of items the monkey is starting with
            let item_list = caps.name("item_list").unwrap().as_str();
            let item_list = item_list
                .split(", ")
                .map(|num| num.parse::<u64>().unwrap())
                .collect::<VecDeque<_>>();
            // println!("Item list: {item_list}", );

            // Get the operation the monkey does on inspection
            let operation = caps.name("operation").unwrap().as_str();
            let operation = match operation.split_once(" ").expect("Bad operation definition") {
                // Operators can only be + or *
                ("+", "old") => Operation::Multiply(2),
                ("*", "old") => Operation::Square(),
                ("+", num) => Operation::Add(num.parse::<u64>().unwrap().clone()),
                ("*", num) => Operation::Multiply(num.parse::<u64>().unwrap()),
                _ => panic!("Invalid operation format")
            };
            // println!("Operation: {operation}", );

            // Get the divisor used by the monkey for its test
            let divisor_test = caps.name("divisor_test").unwrap().as_str();
            let divisor_test = divisor_test.parse::<u64>().expect("Bad divisor test number");
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
                starting_items: item_list,
                operation,
                divisor_test,
                true_monkey,
                false_monkey,
            }
        })
        .collect::<Vec<Monkey>>()
}

/// Given a list of monkey notes, calculate the total amount of monkey business that happens over the course of the given number rounds
fn calulate_monkey_business(monkey_notes: Vec<Monkey>, round_count: u32, reduce_worry_after_inspect: bool) -> u64 {
    // Scan through the monkey_notes, pulling out the items into a separate list, and colleting a vector of the item indices for each monkey
    let mut full_item_list = Vec::<ItemWorryRepresentation>::new();

    // Create a mutable list of each monkey's items (the indexes to use to get them) that we can use to keep track of which they're holding throughout the rounds
    let mut monkey_items = monkey_notes
        .iter()
        .scan(0usize, |item_num, monkey| {
            // Return the Deque of item indices that this monkey is holding
            Some(
                monkey.starting_items
                .iter()
                .map(|starting_worry| {
                    // Add item to the full item list
                    let item_worry = if reduce_worry_after_inspect {
                        ItemWorryRepresentation::Value(*starting_worry)
                    }
                    else {
                        ItemWorryRepresentation::OpertionChain {
                            base_worry: *starting_worry,
                            operation_chain: Vec::new(),
                            modulo_cache: HashMap::new(),
                        }
                    };
                    full_item_list.push(item_worry);

                    // Increment the item_num
                    let item_index = *item_num;
                    *item_num += 1;

                    // Return the index of the item added
                    item_index
                })
                .collect::<VecDeque<_>>()
            )
        })
        .collect::<Vec<VecDeque<_>>>();

    // A vector to keep track of the number of times each monkey has inspected an item
    let mut inspect_counts = vec![0; monkey_notes.len()];

    // Play out round_count number of rounds of monkey business
    for _ in 0..round_count {
        // Go through each monkey's turn
        for (monkey_num, monkey) in monkey_notes.iter().enumerate() {
            // println!("Monkey {monkey_num}:");
            // For each element in their inventory...
            while let Some(item_index) = monkey_items.get_mut(monkey_num).unwrap().pop_front() {
                let worry_level = full_item_list.get_mut(item_index).unwrap();
                // Inspect the item - apply the operation, modifying the worry_level accordingly
                match worry_level {
                    ItemWorryRepresentation::Value(ref mut worry_level) => worry_value_monkey_inspect(worry_level, monkey),
                    ItemWorryRepresentation::OpertionChain {
                        base_worry: _,
                        ref mut operation_chain,
                        modulo_cache: _
                    } => operation_chain.push(monkey.operation.clone()),
                }

                // Record that an item was inspected by this monkey
                *inspect_counts.get_mut(monkey_num).unwrap() += 1;

                // Perform relief division on worry_level seeing that the item wasn't broken
                if reduce_worry_after_inspect {
                    if let ItemWorryRepresentation::Value(ref mut worry_level) = worry_level {
                        // println!("    Monkey gets bored with item. Worry level is divided by 3 to {worry_level}.");
                        *worry_level /= 3;
                    }
                }

                // Perform monkey test and send the item to the appropriate monkey depending on the result
                let is_divisible = match worry_level {
                    ItemWorryRepresentation::Value(worry_level) => *worry_level % monkey.divisor_test == 0,
                    ItemWorryRepresentation::OpertionChain {
                        base_worry,
                        operation_chain,
                        modulo_cache,
                    } => modular_reduced_worry_test(*base_worry, &operation_chain, modulo_cache, monkey.divisor_test),
                };
                let dest_monkey = if is_divisible {
                    monkey.true_monkey
                }
                else {
                    monkey.false_monkey
                };
                // println!("    Item with worry level {worry_level} is thrown to monkey {dest_monkey} after checking divisibility of {}.", monkey.divisor_test);
                monkey_items.get_mut(dest_monkey).unwrap().push_back(item_index);
            }
        }
    }

    // Find the two most inspective monkeys after all the rounds and multiply their inspectiveness for the total monkey business
    // println!("Inspect counts: {:?}", inspect_counts);
    inspect_counts.sort();
    inspect_counts.iter().rev().cloned().take(2).product()
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

/// Calculate the total amount of monkey business that happens over the course of 20 rounds, reducing worry after safe inspection
fn part_one(monkey_notes: &str) -> u64 {
    // Parse the notes about each monkey into a vector of Monkey structs
    let monkey_notes = parse_monkey_notes(monkey_notes);

    // Get the total monkey business after 20 rounds, while being reducing your worry after 
    calulate_monkey_business(monkey_notes, 20, true) 
}

/// Inspect the item with the given worry-level direct value by performing this monkey's operation on it 
fn worry_value_monkey_inspect(worry_level: &mut u64, monkey: &Monkey) {
    // println!("  Monkey inspects an item with a worry level of {worry_level}.");
    *worry_level = match monkey.operation {
        Operation::Add(num) => *worry_level + num,
        Operation::Multiply(num) => *worry_level * num,
        Operation::Square() => (*worry_level).pow(2),
    };
    // println!("    After operation {:?}, worry level becomes {worry_level}", monkey.operation);
}

// endregion

// region: Part Two

#[test]
fn part_two_example_test() {
    // Read in the example monkey notes trimming any surrounding whitespace
    let example_monkey_notes = include_str!("../inputs/example").trim();

    // Get the total monkey business over 10000 rounds of the example monkey input from the question
    let part_two_result = part_two(example_monkey_notes);

    // Check if the example yields the same result as the question describes
    assert_eq!(part_two_result, 2713310158);
}

/// Calculate the total amount of monkey business that happens over the course of 10000 rounds, not reducing worry after safe inspection
fn part_two(monkey_notes: &str) -> u64 {
    // Parse the notes about each monkey into a vector of Monkey structs
    let monkey_notes = parse_monkey_notes(monkey_notes);

    // Get the total monkey business after 20 rounds, while being reducing your worry after 
    calulate_monkey_business(monkey_notes, 10000, false) 
}

/// Check whether the large worry number represented by the base_worry and the chain of operations performed on it is divisible by the divisor
fn modular_reduced_worry_test(base_worry: u64, operation_chain: &Vec<Operation>, modulo_cache: &mut HashMap<u64, (usize, u64)>, divisor: u64) -> bool {
    // Check if there has already been a modulo calculation for this divisor 
    let (last_done_index, initial_worry) = modulo_cache
        .get(&divisor)
        .copied()
        .unwrap_or((0, base_worry));

    // Get the modulo of the large number represented by the base_worry and the chain of operations performed on it
    let mod_result = operation_chain
        .iter()
        .skip(last_done_index)
        .fold(initial_worry, |previous_mod_result, operation| {
            match operation {
                Operation::Add(add_amount) => (previous_mod_result % divisor + *add_amount % divisor) % divisor,
                Operation::Multiply(multiply_amount) => (previous_mod_result % divisor * *multiply_amount % divisor) % divisor,
                Operation::Square() => (previous_mod_result % divisor).pow(2) % divisor,
            }
        });

    // Record the mod_result in the modulo_cache so the operation doesn't have to recompute all of this again
    modulo_cache.insert(divisor, (operation_chain.len(), mod_result));

    // Return whether the number is divided by the divisor by checking if mod_result == 0
    mod_result == 0
}

// endregion
