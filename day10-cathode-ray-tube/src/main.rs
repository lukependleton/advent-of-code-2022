use std::iter::repeat;

fn main() {
    // Read in the cpu instructions from the question trimming any surrounding whitespace
    let question_cpu_instructions = include_str!("../inputs/question").trim();

    // * Part One
    // Determine the sum of the signal strength across important clock cycles in the input from the question
    let part_one_result = part_one(question_cpu_instructions);
    println!("Part One:\n  The sum of the important signal strengths is: {part_one_result}");

    // * Part Two
    // Get the screen output of the CRT after processing the cpu_instructions from the input from the question
    let cathode_ray_screen_output = part_two(question_cpu_instructions);
    println!("Part Two:\n  The output of the CRT screen is:\n{cathode_ray_screen_output}");
}

// region: Helpers

enum Instruction {
    AddX(i32),
    Noop,
}

/// Given the input `cpu_instructions` str, parse each line to get a vector of Instructions
fn parse_cpu_instructions(cpu_instructions: &str) -> Vec<Instruction> {
    cpu_instructions
        .split('\n')
        .map(|line| {
            let mut message_split = line.split(' ');
            match (message_split.next(), message_split.next()) {
                (Some("addx"), Some(add_value)) => {
                    Instruction::AddX(add_value.parse::<i32>().expect("Invalid addx arg - needs to be a valid number"))
                },
                (Some("noop"), None) => Instruction::Noop,
                _ => panic!("Invalid command setup: '{line}'"),
            }
        })
        .collect::<Vec<_>>()
}

/// Process the given instructions returning the value of the x register at every clock cycle it takes to complete all of them
/// (x register starts with value 1)
fn process_instructions(cpu_instructions: Vec<Instruction>) -> Vec<i32> {
    cpu_instructions
        .iter()
        .scan(1, |register_x, instruction| {
            match instruction {
                Instruction::AddX(add_value) => {
                    // Record what the initial value for register_x was at the beginning of processing the instruction
                    let current_x_value = *register_x;

                    // Update the register_x based on the argument to the addx command
                    *register_x += add_value;

                    // Return the what register x was during the clock cycles it took to process this addx command
                    Some(repeat(current_x_value).take(2))
                },
                Instruction::Noop => Some(repeat(*register_x).take(1)),
            }
        })
        .flatten()
        .collect::<Vec<_>>()
}

// endregion

// region: Part One

#[test]
fn part_one_example_test() {
    // Read in the example cpu instructions trimming any surrounding whitespace
    let example_cpu_instructions = include_str!("../inputs/example").trim();

    // Determine the sum of the signal strength across important clock cycles in the example input from the question
    let part_one_result = part_one(example_cpu_instructions);

    // Check if the example yields the same result as the question describes
    assert_eq!(part_one_result, 13140);
}

/// Determine the sum of the signal strength across important clock cycles given the set up `cpu_instructions`
fn part_one(cpu_instructions: &str) -> i32 {
    // Parse the input str into a vector of instructions
    let parsed_cpu_instructions = parse_cpu_instructions(cpu_instructions);

    // Evaluate the instructions, recording the value of the x register for each cycle
    let register_x_values = process_instructions(parsed_cpu_instructions);

    // Filter on the important clock cycles and calculate the signal strength at each
    let important_singal_strengths = register_x_values
        .iter()
        .enumerate()
        .filter_map(|(i, register_value)| {
            // The number of the clock cycle is one more than its index in the vector (it starts at 1)
            let cycle_num = (i + 1) as i32;
            if cycle_num % 40 - 20 == 0 {
                Some(cycle_num * *register_value)
            }
            else {
                None
            }
        })
        .collect::<Vec<_>>();

    // Return the sum of the important signal strengths
    important_singal_strengths.iter().sum()
}

// endregion

// region: Part Two

#[test]
fn part_two_example_test() {
    // Read in the example cpu instructions trimming any surrounding whitespace
    let example_cpu_instructions = include_str!("../inputs/example").trim();

    // Get the screen output of the CRT after processing the cpu_instructions from the example input from the question
    let part_two_result = part_two(example_cpu_instructions);

    // Check if the example yields the same result as the question describes
    assert_eq!(part_two_result, String::from("\
        ##..##..##..##..##..##..##..##..##..##..\n\
        ###...###...###...###...###...###...###.\n\
        ####....####....####....####....####....\n\
        #####.....#####.....#####.....#####.....\n\
        ######......######......######......####\n\
        #######.......#######.......#######.....\
    "));
}

/// Execute the cpu_instructions rendering the resulting CRT image of the screen to the return String
fn part_two(cpu_instructions: &str) -> String {
    // Parse the input str into a vector of instructions
    let parsed_cpu_instructions = parse_cpu_instructions(cpu_instructions);

    // Evaluate the instructions, recording the value of the x register for each cycle
    let register_x_values = process_instructions(parsed_cpu_instructions);

    // Determine the pixels being drawn by the CRT at each clock cycle
    let crt_pixels = register_x_values
        .iter()
        .enumerate()
        .map(|(i, register_x_value)| {
            let screen_index = (i % 40) as i32;
            // If i is drawing one of the picels of the sprite, draw the lit pixel
            if screen_index.abs_diff(*register_x_value) <= 1 {
                '#'
            }
            // Otherwise, draw the dark pixel
            else {
                '.'
            }
        })
        .collect::<Vec<char>>();

    // Render the pixels of the crt (a screen in 40 pixels wide) and return the resulting String
    crt_pixels
        .chunks(40)
        .map(|screen_row| {
            screen_row.iter().collect::<String>()
        })
        .collect::<Vec<_>>()
        .join("\n")
}

// endregion
