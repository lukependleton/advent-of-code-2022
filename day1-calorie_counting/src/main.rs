use std::fs;

fn main() {
    // TODO: paths...make them relative to the project directory (or this file) - not where the program is called from
    // TODO: if we did want to do that, I think we would need to do something like this:
    //  - https://doc.rust-lang.org/std/macro.include_str.html

    // * Part One
    // Find the largest calorie count among the elves
    let largest_calorie_count = part_one("inputs/question");
    println!("Part One:\n  The largest calorie count is: {largest_calorie_count}");

    // * Part Two
    // Find the sum of the calorie counts of the top three elves' counts
    let top_three_largest_total = part_two("inputs/question");
    println!("Part Two:\n  The sum of the top three largest calorie counts is: {top_three_largest_total}");
}


// region: Helpers

fn get_elf_calorie_counts_from_string(inventory: &str) -> Vec<u32> {
    // Split full list into each elf's list by splitting on the new lines
    let elf_inventory_list = inventory.split("\n\n");

    // For each, find their calorie sum
    elf_inventory_list
        .map(
            |elf_inventory|
            elf_inventory
                .split('\n')
                .map(
                    |line| line.parse::<u32>().expect(&format!("Failed to parse line: '{line}'"))
                )
                .sum()
        ).collect::<Vec<u32>>()
}

// endregion


// * Part One

// region: Day 1 - Part One

#[test]
fn part_one_example_test() {
    // Find the largest calorie count among the elves in the example input from the question
    let largest_calorie_count = part_one("inputs/example");

    // Check if the example yields the same result as the question describes
    assert_eq!(largest_calorie_count, 24000);
}

fn part_one(filepath: &str) -> u32 {
    // Read input from file
    // TODO: potentially trim the input file to remove whitespace like newlines from the ends
    let food_inventory = fs::read_to_string(filepath).expect("Failed to read in the file");

    // Find the largest calorie count among the elves
    get_largest_elf_calorie_count(food_inventory.trim())
}

fn get_largest_elf_calorie_count(inventory: &str) -> u32 {
    // Get the total calorie counts of each elf
    let calorie_counts = get_elf_calorie_counts_from_string(inventory);

    // Return the biggest, panicking if it couldn't find one
    *calorie_counts.iter().max().expect("Couldn't find a max calorie count")
}

// endregion


// * Part Two

// region: Day 1 - Part Two

#[test]
fn part_two_example_test() {
    // Find the sum of the calorie counts of the top three elves' counts in the example input from the question
    let top_three_largest_total = part_two("inputs/example");

    // Check if the example yields the same result as the question describes
    assert_eq!(top_three_largest_total, 45000);
}

fn part_two(filepath: &str) -> u32 {
    // Read input from file
    let food_inventory = fs::read_to_string(filepath).expect("Failed to read in the file");

    // Find the sum of the calorie counts of the top three elves' counts
    get_top_three_calorie_counts_total(food_inventory.trim())
}

fn get_top_three_calorie_counts_total(inventory: &str) -> u32 {
    // Get the total calorie counts of each elf
    let calorie_counts = get_elf_calorie_counts_from_string(inventory);

    // Find the sum of the top three counts in the vector
    let top_three = calorie_counts.iter().fold(
        vec![0u32; 3],
        |top_three_acc, count| {
            // Concat the current top three and the current element to consider all four of these at once
            let mut top_three_acc = top_three_acc.clone();
            top_three_acc.push(*count);

            // Remove the minimum element of these four counts
            let (min_index, _) = top_three_acc.iter().enumerate().min_by_key(|&(_, count)| count)
                .expect("Couldn't find a minimum element when determining the accumulator");
            top_three_acc.remove(min_index);

            // Return the top three of these four as the new accumulator after having seen this elem
            top_three_acc
        }
    );

    // Total the top three calorie counts
    top_three.iter().sum()
}

// endregion
