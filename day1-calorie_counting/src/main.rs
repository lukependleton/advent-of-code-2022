fn main() {
    // Read in the input food inventory from the question trimming any surrounding whitespace
    let food_inventory = include_str!("../inputs/question").trim();

    // * Part One
    // Find the largest calorie count among the elves
    let largest_calorie_count = part_one(food_inventory);
    println!("Part One:\n  The largest calorie count is: {largest_calorie_count}");

    // * Part Two
    // Find the sum of the calorie counts of the top three elves' counts
    let top_three_largest_total = part_two(food_inventory);
    println!("Part Two:\n  The sum of the top three largest calorie counts is: {top_three_largest_total}");
}


// region: Helpers

fn get_elf_calorie_counts_from_str(inventory: &str) -> Vec<u32> {
    // Split full list into each elf's list by splitting on the new lines
    let elf_inventory_list = inventory.split("\n\n");

    // For each elf, find their calorie sum
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


// region: Part One

#[test]
fn part_one_example_test() {
    // Read in the example food inventory trimming any surrounding whitespace
    let example_food_inventory = include_str!("../inputs/example").trim();

    // Find the largest calorie count among the elves in the example input from the question
    let largest_calorie_count = part_one(example_food_inventory);

    // Check if the example yields the same result as the question describes
    assert_eq!(largest_calorie_count, 24000);
}

/// Find the largest calorie count among the elves given the `food_inventory` of all the elves
fn part_one(food_inventory: &str) -> u32 {
    // Get the total calorie counts of each elf
    let calorie_counts = get_elf_calorie_counts_from_str(food_inventory);

    // Return the biggest, panicking if it couldn't find one
    *calorie_counts.iter().max().expect("Couldn't find a max calorie count")
}

// endregion


// region: Part Two

#[test]
fn part_two_example_test() {
    // Read in the example food inventory trimming any surrounding whitespace
    let example_food_inventory = include_str!("../inputs/example").trim();

    // Find the sum of the calorie counts of the top three elves' counts in the example input from the question
    let top_three_largest_total = part_two(example_food_inventory);

    // Check if the example yields the same result as the question describes
    assert_eq!(top_three_largest_total, 45000);
}

/// Find the sum of the calorie counts of the top three elves' counts given the `food_inventory` of all the elves
fn part_two(food_inventory: &str) -> u32 {
    // Get the total calorie counts of each elf
    let calorie_counts = get_elf_calorie_counts_from_str(food_inventory);

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
