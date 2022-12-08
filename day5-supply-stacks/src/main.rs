use std::collections::VecDeque;

fn main() {
    // Read in the supply stack input from the question
    let question_supply_stack_input = include_str!("../inputs/question");

    // * Part One
    // Find the crates that will be at the top of the stacks given the starting stacks and rearrangement procedure for part one
    let top_crates = part_one(question_supply_stack_input);
    println!("Part One:\n  The top crates in order after the part one rearrangement are: {top_crates}");

    // * Part Two
    // Find the crates that will be at the top of the stacks given the starting stacks and rearrangement procedure for part two
    let top_crates = part_two(question_supply_stack_input);
    println!("Part Two:\n  The top crates in order after the part two rearrangement are: {top_crates}");
}

// region: Helpers

#[derive(Debug)]
struct RearrangementStep {
    num_crates: u32,
    source_stack: usize,
    dest_stack: usize,
}

fn parse_starting_crate_stacks(starting_crate_stacks_str: &str) -> Vec<VecDeque<char>> {
    // Split the input by new lines and parse the crates into their respective char IDs for each horizontal slice
    let mut horizontal_slice_crate_stacks = starting_crate_stacks_str
        .split('\n')
        .map(|line| {
            // Parse the crate chunks into a vector of chars
            line
                .chars()
                .collect::<Vec<char>>()
                .chunks(4)
                .map(|crate_chunk| {
                    // Erroring this index is fine as that would mean an invalid input...
                    crate_chunk[1]
                })
                .collect::<Vec<char>>()
        })
        .collect::<Vec<Vec<char>>>();

    // Remove the last element of the horizontal slices
    let num_stacks = horizontal_slice_crate_stacks.pop().unwrap().len();
    // println!("The number of stacks is {num_stacks}");
    
    // println!("The tallest stack to start is {}", horizontal_slice_crate_stacks.len());

    // println!("The slices:\n{:?}", horizontal_slice_crate_stacks);

    // Fold the parsed list of crate horizontal slices into crate stacks
    horizontal_slice_crate_stacks
        .iter()
        .fold(vec![VecDeque::<char>::new(); num_stacks], |mut acc, crate_stack_slice| {
            // Add the crates to their appropriate stacks (if they exist)
            crate_stack_slice
                .iter()
                .enumerate()
                .for_each(|(i, crate_char)| {
                    if *crate_char != ' ' {
                        acc[i].push_front(*crate_char)
                    }
                });

            // Return the accumulator
            acc
        })
}

fn parse_supply_stack_input(supply_stack_input: &str) -> (Vec<VecDeque<char>>, Vec<RearrangementStep>) {
    // Split at double new line to get the starting_crate_stacks and the rearrangement_procedure
    let (starting_crate_stacks, rearrangement_procedure) = supply_stack_input
        .split_once("\n\n")
        .expect("Needs both sections crate stack input, but input seems invlaid");

    // Parse the starting crate stacks into a vector of stacks
    let starting_crate_stacks = parse_starting_crate_stacks(starting_crate_stacks);

    // Parse the rearrangement procedure into a list of steps
    let rearrangement_procedure = rearrangement_procedure
        .trim()
        .split('\n')
        .map(|line| {
            // Parse the line into its
            let instruction_split = line.split(' ').collect::<Vec<&str>>();
            match instruction_split[..] {
                ["move", num, "from", source, "to", dest, ..] => RearrangementStep {
                    num_crates: num.parse::<u32>().unwrap(),
                    source_stack: source.parse::<usize>().unwrap() - 1,
                    dest_stack: dest.parse::<usize>().unwrap() - 1,
                },
                _ => panic!("Bad rearrangement instructions..."),
            }
        })
        .collect::<Vec<RearrangementStep>>();

    // Return the parsed pieces
    (starting_crate_stacks, rearrangement_procedure)
}

fn perform_supply_stack_rearrangement<F>(supply_stack_input: &str, rearrangement_logic: F) -> String where
    F: Fn(&mut Vec<VecDeque<char>>, &RearrangementStep) {
    // Parse the supply stack input
    let (mut crate_stacks, rearrangement_procedure) = parse_supply_stack_input(supply_stack_input);

    // Execute each step of the rearrangement procedure
    rearrangement_procedure
        .iter()
        .for_each(|rearrangement_step| {
            rearrangement_logic(&mut crate_stacks, rearrangement_step)
        });

    // Return the top crates of the stacks as a String
    crate_stacks
        .iter()
        .map(|stack| *stack.back().unwrap())
        .collect::<String>()
}

// endergion

// region: Part One

#[test]
fn part_one_example_test() {
    // Read in the example supply stack input trimming any surrounding whitespace
    let example_supply_stack_input = include_str!("../inputs/example");

    // Find the crates that will be at the top of the stacks given the example starting stacks and rearrangement procedure from part one
    let top_crates = part_one(example_supply_stack_input);

    // Check if the example yields the same result as the question describes
    assert_eq!(top_crates, String::from("CMZ"));
}

// Find the number of containing assignment pairs from the `cleanup_assignments`
fn part_one(supply_stack_input: &str) -> String {
    // Perform the supply stack rearrangement considering part one's understanding of the crane
    perform_supply_stack_rearrangement(supply_stack_input, part_one_rearrangement_logic)
}

fn part_one_rearrangement_logic(crate_stacks: &mut Vec<VecDeque<char>>, rearrangement_step: &RearrangementStep) {
    // Some debug output
    // println!("Stacks currently: {:?}", crate_stacks);
    // println!("Doing step: {rearrangement_step:?}");

    // Perform the rearrangement
    (0..rearrangement_step.num_crates).for_each(|_| {
        let crate_to_move = crate_stacks[rearrangement_step.source_stack].pop_back().unwrap();
        crate_stacks[rearrangement_step.dest_stack].push_back(crate_to_move);
    })
}

// endregion

// region: Part Two

#[test]
fn part_two_example_test() {
    // Read in the example supply stack input trimming any surrounding whitespace
    let example_supply_stack_input = include_str!("../inputs/example");

    // Find the crates that will be at the top of the stacks given the example starting stacks and rearrangement procedure from part two
    let top_crates = part_two(example_supply_stack_input);

    // Check if the example yields the same result as the question describes
    assert_eq!(top_crates, String::from("MCD"));
}

// Find the number of containing assignment pairs from the `cleanup_assignments`
fn part_two(supply_stack_input: &str) -> String {
    // Perform the supply stack rearrangement considering part two's understanding of the crane
    perform_supply_stack_rearrangement(supply_stack_input, part_two_rearrangement_logic)
}

fn part_two_rearrangement_logic(crate_stacks: &mut Vec<VecDeque<char>>, rearrangement_step: &RearrangementStep) {
    // Create a temporay stack to hold the crates while they are moving
    let mut crane_stack = VecDeque::<char>::new();
    // Perform the rearrangement
    (0..rearrangement_step.num_crates).for_each(|_| {
        let crate_to_move = crate_stacks[rearrangement_step.source_stack].pop_back().unwrap();
        crane_stack.push_back(crate_to_move);
    });
    (0..rearrangement_step.num_crates).for_each(|_| {
        let crate_to_move = crane_stack.pop_back().unwrap();
        crate_stacks[rearrangement_step.dest_stack].push_back(crate_to_move);
    });
}

// endregion
