fn main() {
    // Read in the cleanup_assignment from the question trimming any surrounding whitespace
    let question_cleanup_assignment = include_str!("../inputs/question").trim();

    // * Part One
    // Find the number of containing assignment pairs from the input from the question
    let shared_assignments = part_one(question_cleanup_assignment);
    println!("Part One:\n  The number of elf pairs where one cleanup assignment fully contains the other is: {shared_assignments}");

    // * Part Two
    // Find the number of overlapping assignment pairs from the input from the question
    let overlapping_assignments = part_two(question_cleanup_assignment);
    println!("Part Two:\n  The number of elf pairs where one cleanup assignment overlaps the other is: {overlapping_assignments}");
}

// region: Helpers

/// Parse the pairs of cleanup assignment ranges for the elves
fn get_elf_pair_assignment_ranges(cleanup_assignments: &str) -> Vec<((i32, i32), (i32, i32))> {
    cleanup_assignments
        .split('\n')
        .map(|line| {
            let (first_elf, second_elf) = line.split_once(',')
                .expect("Missing a comma - bad input");
            (get_range(first_elf), get_range(second_elf))
        })
        .collect::<Vec<((i32, i32), (i32, i32))>>()
}

/// Parse the space-separated numbers in the given `range_str` as a range of two ints
fn get_range(range_str: &str) -> (i32, i32) {
    // We are expecting the range to be space-separated
    let (lower, upper) = range_str.split_once('-').unwrap();
    // For now, let's just read them into a pair of nums as opposed to a range...
    (lower.parse::<i32>().unwrap(), upper.parse::<i32>().unwrap())
}

/// Parse the cleanup assignment input, checks whether each pair follows the given predicate, and returns the number of them
fn get_num_applicable_assignment_pairs<F>(cleanup_assignments: &str, condition: F) -> u32 where
    F: Fn(&(i32, i32), &(i32, i32)) -> bool {
    // Parse the cleanup_assignments string into the elf pairs' individual cleanup assignments ranges
    let elf_pair_assignment_ranges = get_elf_pair_assignment_ranges(cleanup_assignments);

    // For each assignment, determine if the pair meets the given condition
    let assignment_pair_truthinesses = elf_pair_assignment_ranges
        .iter()
        .map(|(assign_1, assign_2)| {
            // Check if pair meets the condition
            let pair_truthiness = condition(assign_1, assign_2);
            pair_truthiness as u32
        })
        .collect::<Vec<u32>>();

    // Return the total number of pairs that fulful the predicate
    assignment_pair_truthinesses.iter().sum()
}

// endregion

// region: Part One

#[test]
fn part_one_example_test() {
    // Read in the example cleanup_assignment trimming any surrounding whitespace
    let example_cleanup_assignments = include_str!("../inputs/example").trim();

    // Find the number of containing assignment pairs from the example input from the question
    let shared_assignments = part_one(example_cleanup_assignments);

    // Check if the example yields the same result as the question describes
    assert_eq!(shared_assignments, 2);
}

// Find the number of containing assignment pairs from the `cleanup_assignments`
fn part_one(cleanup_assignments: &str) -> u32 {
    // For each assignment, determine if one assignment range contains the other
    get_num_applicable_assignment_pairs(cleanup_assignments, |assign_1, assign_2| {
        range_contains_range(assign_1, assign_2) || range_contains_range(assign_2, assign_1)
    })
}

/// Check if either fully contains the other
fn range_contains_range(range_1: &(i32, i32), range_2: &(i32, i32)) -> bool {
    range_1.0 <= range_2.0 && range_1.1 >= range_2.1
}

// endregion

// region: Part Two

#[test]
fn part_two_example_test() {
    // Read in the example cleanup_assignment trimming any surrounding whitespace
    let example_cleanup_assignments = include_str!("../inputs/example").trim();

    // Find the number of overlapping assignment pairs from the example input from the question
    let overlapping_assignments = part_two(example_cleanup_assignments);

    // Check if the example yields the same result as the question describes
    assert_eq!(overlapping_assignments, 4);
}

// Find the number of overlapping assignment pairs from the `cleanup_assignments`
fn part_two(cleanup_assignments: &str) -> u32 {
    // For each assignment, determine if one assignment range overlaps the other at all
    get_num_applicable_assignment_pairs(cleanup_assignments, range_overlaps_range)
}

fn range_overlaps_range(range_1: &(i32, i32), range_2: &(i32, i32)) -> bool {
    range_1.0 <= range_2.1 && range_2.0 <= range_1.1
}

// endregion
