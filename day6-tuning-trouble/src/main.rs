use std::collections::{VecDeque, HashSet};

fn main() {
    // Read in the datastream buffer from the question trimming any surrounding whitespace
    let question_datastream_buffer = include_str!("../inputs/question").trim();

    // * Part One
    // Find the index of the start-of-packet marker in the input from the question
    let start_of_packet_index = part_one(question_datastream_buffer);
    println!("Part One:\n  The index of the start-of-packet marker is: {start_of_packet_index}");

    // * Part Two
    // Find the index of the start-of-message marker in the input from the question
    let start_of_message_index = part_two(question_datastream_buffer);
    println!("Part Two:\n  The index of the start-of-message marker is: {start_of_message_index}");
}

// region: Helpers

/// Given an input `datastream_buffer`, find the point at which the previous `num_distict` characters were all disinct
fn get_index_of_n_distinct_chars(num_distict: usize, datastream_buffer: &str) -> usize {
    let mut datastream_buffer_iter = datastream_buffer.chars().enumerate();

    // There's probably a better data structure that will make checking if there are duplicates easier...
    let mut last_n = VecDeque::<char>::new();

    // Push first n - 1 chars into last_n
    for _ in 0..(num_distict - 1) {
        last_n.push_back(datastream_buffer_iter.next().unwrap().1);
    }

    match datastream_buffer_iter
        .try_fold(last_n, |mut acc, (i, code)| {
            // Push next char to last_n queue
            acc.push_back(code);

            // Check if last_n are all unique
            if acc.iter().cloned().collect::<HashSet<char>>().len() == num_distict {
                // Return the i value
                return Err(i);
            }

            // Pop the oldest element from the last_n queue
            acc.pop_front().expect("The last_n queue was unexpectedly empty...");

            // Return last_n (with now n - 1 elems in it)
            Ok(acc)
        }) {
            Ok(_) => panic!("It doesn't seem that we found a position where there were four previous unique elems..."),
            Err(i) => i + 1,
        }
}

// endregion

// region: Part One

#[test]
fn part_one_example_1_test() {
    // Read in the example datastream buffer trimming any surrounding whitespace
    let example_datastream_buffer = include_str!("../inputs/example").trim();

    // Find the index of the start-of-packet marker in the example input from the question
    let start_of_packet_index = part_one(example_datastream_buffer);

    // Check if the example yields the same result as the question describes
    assert_eq!(start_of_packet_index, 7);
}

#[test]
fn part_one_example_2_test() {
    assert_eq!(part_one(include_str!("../inputs/example2")), 5);
}

#[test]
fn part_one_example_3_test() {
    assert_eq!(part_one(include_str!("../inputs/example3")), 6);
}

#[test]
fn part_one_example_4_test() {
    assert_eq!(part_one(include_str!("../inputs/example4")), 10);
}

#[test]
fn part_one_example_5_test() {
    assert_eq!(part_one(include_str!("../inputs/example5")), 11);
}

/// Find the start-of-packet marker by finding the index at which the 4 previous characters are all unique
fn part_one(datastream_buffer: &str) -> usize {
    get_index_of_n_distinct_chars(4, datastream_buffer)
}

// endregion

// region: Part Two

#[test]
fn part_two_example_1_test() {
    // Read in the example datastream buffer trimming any surrounding whitespace
    let example_datastream_buffer = include_str!("../inputs/example").trim();

    // Find the index of the start-of-message marker in the example input from the question
    let start_of_message_index = part_two(example_datastream_buffer);

    // Check if the example yields the same result as the question describes
    assert_eq!(start_of_message_index, 19);
}

#[test]
fn part_two_example_2_test() {
    assert_eq!(part_two(include_str!("../inputs/example2")), 23);
}

#[test]
fn part_two_example_3_test() {
    assert_eq!(part_two(include_str!("../inputs/example3")), 23);
}

#[test]
fn part_two_example_4_test() {
    assert_eq!(part_two(include_str!("../inputs/example4")), 29);
}

#[test]
fn part_two_example_5_test() {
    assert_eq!(part_two(include_str!("../inputs/example5")), 26);
}

/// Find the start-of-message marker by finding the index at which the 14 previous characters are all unique
fn part_two(datastream_buffer: &str) -> usize {
    get_index_of_n_distinct_chars(14, datastream_buffer)
}

// endregion
