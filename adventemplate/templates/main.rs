fn main() {
    // Read in the {{ input_structure_name | replace(from="_", to=" ") }} from the question trimming any surrounding whitespace
    let question_{{ input_structure_name }} = include_str!("../inputs/question").trim();

    // * Part One
    // <Whatever part one is doing>
    let part_one_result = part_one(question_{{ input_structure_name }});
    println!("Part One:\n  The result is: {part_one_result}");

    // * Part Two
    // <Whatever part two is doing>
    let part_two_result = part_two(question_{{ input_structure_name }});
    println!("Part Two:\n  The result is: {part_two_result}");
}

// region: Helpers

fn parse_{{ input_structure_name }}({{ input_structure_name }}: &str) -> Vec<()> {
    vec![()]
}

// endregion

// region: Part One

#[test]
fn part_one_example_test() {
    // Read in the example {{ input_structure_name | replace(from="_", to=" ") }} trimming any surrounding whitespace
    let example_{{ input_structure_name }} = include_str!("../inputs/example").trim();

    // <Part One goal> in the example input from the question
    let part_one_result = part_one(example_{{ input_structure_name }});

    // Check if the example yields the same result as the question describes
    assert_eq!(part_one_result, 0);
}

/// <Part One goal>
fn part_one({{ input_structure_name }}: &str) -> u32 {
    // <Describe what parsing the input is doing>
    let parsed_{{ input_structure_name }} = parse_{{ input_structure_name }}({{ input_structure_name }});

    // The rest...
    0
}

// endregion

// region: Part Two

#[test]
fn part_two_example_test() {
    // Read in the example {{ input_structure_name | replace(from="_", to=" ") }} trimming any surrounding whitespace
    let example_{{ input_structure_name }} = include_str!("../inputs/example").trim();

    // <Part Two goal> in the example input from the question
    let part_two_result = part_two(example_{{ input_structure_name }});

    // Check if the example yields the same result as the question describes
    assert_eq!(part_two_result, 0);
}

/// <Part Two goal>
fn part_two({{ input_structure_name }}: &str) -> u32 {
    // <Describe what parsing the input is doing>
    let parsed_{{ input_structure_name }} = parse_{{ input_structure_name }}({{ input_structure_name }});

    // The rest...
    0
}

// endregion
