use std::collections::HashSet;

fn main() {    
    // Read in the rucksack item list from the question trimming any surrounding whitespace
    let question_rucksack_items = include_str!("../inputs/question").trim();

    // * Part One
    // Determine the sum of the priorities of the misplaced items in the rucksack item list from the question
    let sum_of_priorities = part_one(question_rucksack_items);
    println!("Part One:\n  The sum of the misplaced items' priorities is: {sum_of_priorities}");
    
    // * Part Two
    // Determine the sum of the priorities of badges among the elf groups in the rucksack item list from the question
    let sum_of_group_badge_priorities = part_two(question_rucksack_items);
    println!("Part Two:\n  The sum of the elf groups' badge priorities is: {sum_of_group_badge_priorities}");
}

// region: Helpers

// Just a little python-inspired syntactic sugar :)
fn ord(char: char) -> u32 {
    u32::try_from(char).expect(&format!("The char '{char}' is too big of a value and is invalid input for this question -> basically it's not one of the original 151"))
}

/// Parses the input rucksack item list str into a vector of item priorities (for each rucksack)
fn get_rucksack_item_priorities(rucksack_items: &str) -> Vec<Vec<u32>> {
    rucksack_items
        .split('\n')
        .map(|line| {
            // Get the priority of each item in the line str
            line
                .chars()
                .map(|item_type| {
                    // Get the priority of the given item_type represented by its character
                    get_item_priority(item_type)
                })
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>()
}

/// Gets the item priority associated with the given `item_type`
fn get_item_priority(item_type: char) -> u32 {
    let ascii_value = ord(item_type);
    match ascii_value {
        // * Note: normally, we shouldn't subtract u32's like this as it will panic if there is a negative overflow (underflow?)
        // * In this case though, it never panics because ord('a') and ord('A') are "constant" as the min values of the ranges their respective match arms catch
        // * I personally think it should still be handled more intentionally with subtrating i32s and trying to casting them to u32s at the return of the function with a more helpful expect message
        // * Also, though the return of ord('a') and ord('A') are "constant" right now, if the ord function's impl changed, they might not be
        // If char between a and z, do ord(char) - ord('a') + 1
        97..=122 => ascii_value - ord('a') + 1,
        // If char between A and Z, do ord(char) - ord('A') + 27
        65..=90 => ascii_value - ord('A') + 27,
        _ => panic!("Invalid item type: '{item_type}'")
    }
    // Possibly would change to a u32 at the end if we wanted to remove possibility of failing the subtraction
}

// endregion

// region: Part One

#[test]
fn part_one_example_test() {
    // Read in the example rucksack item list trimming any surrounding whitespace
    let example_rucksack_items = include_str!("../inputs/example").trim();

    // Determine the sum of the priorities of the misplaced items in the example rucksack item list
    let sum_of_priorities = part_one(example_rucksack_items);

    // Check if the example yields the same result as the question describes
    assert_eq!(sum_of_priorities, 157);
}

fn part_one(rucksack_items: &str) -> u32 {
    // Get the list of rucksacks and the item priorities for each of their compartments
    let rucksack_item_priorities = get_rucksack_item_priorities(rucksack_items);

    // Find the items present in both compartments for each rucksack
    let rucksack_organization_culprits = rucksack_item_priorities
        .iter()
        .map(|item_priorities| {
            // Split the bag in half into its two compartments
            assert!(item_priorities.len() % 2 == 0, "Rucksack item list is not even");
            let (compartment_1, compartment_2) = item_priorities.split_at(item_priorities.len() / 2);

            // Get the set intersection of the two compartments of the bag
            let compartment_1 = compartment_1.to_owned().into_iter().collect::<HashSet<u32>>();
            let compartment_2 = compartment_2.to_owned().into_iter().collect::<HashSet<u32>>();
            let intersection = &compartment_1 & &compartment_2;

            // Only check the first intersection, because the input is supposed to only have one
            intersection
                .into_iter()
                .next()
                .expect("The input is supposed to guarentee that there should be exactly one element in the set intersection, but couldn't find one")
                .clone()
        })
        .collect::<Vec<u32>>();

    // Sum all of these
    rucksack_organization_culprits.iter().sum()
}

// endregion

// region: Part two

#[test]
fn part_two_example_test() {
    // Read in the example rucksack item list trimming any surrounding whitespace
    let example_rucksack_items = include_str!("../inputs/example").trim();

    // Determine the sum of the priorities of badges among the elf groups in the example rucksack item list
    let sum_of_group_badge_priorities = part_two(example_rucksack_items);

    // Check if the example yields the same result as the question describes
    assert_eq!(sum_of_group_badge_priorities, 70);
}

fn part_two(rucksack_items: &str) -> u32 {
    // Get the list of rucksacks and the item priorities contained in each
    let rucksack_item_priorities = get_rucksack_item_priorities(rucksack_items);

    // Get the priorities of the badges of each elf group in the list
    let grouped_items_priorities = rucksack_item_priorities
        .chunks(3)
        .map(|elf_group| {
            // Convert the elf vectors into hashsets for faster intersect-checking/contains-checking
            let elf_group_set_iter = elf_group
                .iter()
                .map(|elf_vec| {
                    elf_vec.iter().copied().collect::<HashSet<u32>>()
                });

            // Reduce intersection approach:
            //   Get the intersection of all the elves in the iter by performing a reduce accross the sets with the intersection as the accumulator
            let intersection = elf_group_set_iter
                .reduce(|accum, elem| {
                    // Use the cooler syntax for set intersection using the bitand operator
                    &accum & &elem
                })
                .expect("This elf group has no elves - invalid input");

            // Alternative retains approach:
            //   Get the intersection of all the elves in the iter by filtering out the items that aren't contained in the other elves' sets
            /*
            let mut intersection = elf_group_set_iter.next().expect("This elf group has no elves - invalid input");
            for other_elf in elf_group_set_iter {
                intersection.retain(|item| other_elf.contains(item));
            }
            */

            // Return the intersection of all three elves' items
            intersection.iter().next().expect("No items shared among elves - invalid input").clone()
        })
        .collect::<Vec<u32>>();

        // Sum all of these
        grouped_items_priorities.iter().sum()
}

// endregion
