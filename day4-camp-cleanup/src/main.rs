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

fn get_range(range_str: &str) -> (i32, i32) {
    // We are expecting the range to be space-separated
    let (lower, upper) = range_str.split_once('-').unwrap();
    // For now, let's just read them into a pair of nums as opposed to a range...
    (lower.parse::<i32>().unwrap(), upper.parse::<i32>().unwrap())
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
    // Parse the cleanup_assignments string into the elf pairs' individual cleanup assignments ranges
    let elf_pair_assignment_ranges = get_elf_pair_assignment_ranges(cleanup_assignments);

    // For each assignment, determine if one assignment range contains the other
    let assignment_contains_other_ness = elf_pair_assignment_ranges
        .iter()
        .map(|(assign_1, assign_2)| {
            // Check if either fully contains the other
            let contains_ness_nool = range_contains_range(assign_1, assign_2) || range_contains_range(assign_2, assign_1);
            contains_ness_nool as u32
        })
        .collect::<Vec<u32>>();

    // Return the total number of pairs where one range fully contains the other
    assignment_contains_other_ness.iter().sum()
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
    // Parse the cleanup_assignments string into the elf pairs' individual cleanup assignments ranges
    let elf_pair_assignment_ranges = get_elf_pair_assignment_ranges(cleanup_assignments);

    // For each assignment, determine if one assignment range overlaps the other
    let assignment_overlaps_other_ness = elf_pair_assignment_ranges
        .iter()
        .map(|(assign_1, assign_2)| {
            // Check if one overlaps the other at all
            let overlaps_ness_nool = range_overlaps_range(assign_1, assign_2);
            overlaps_ness_nool as u32
        })
        .collect::<Vec<u32>>();

    // Return the total number of pairs where one range overlaps the other at all
    assignment_overlaps_other_ness.iter().sum()
}

fn range_overlaps_range(range_1: &(i32, i32), range_2: &(i32, i32)) -> bool {
    range_1.0 <= range_2.1 && range_2.0 <= range_1.1
}

// endregion
