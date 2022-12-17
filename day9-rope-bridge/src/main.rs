use std::{collections::HashSet, iter::{self, repeat}, ops::{self, Range}, thread, time::Duration, io::{stdin, Read}};

fn main() {
    // Read in the rope motions from the question trimming any surrounding whitespace
    let question_rope_motions = include_str!("../inputs/larger_example").trim();

    // * Part One
    // Determine the number of unique places the tail of the rope of length 2 visited in the input from the question
    let num_places_visited = part_one(question_rope_motions);
    println!("Part One:\n  The number of unique locations the tail visited for the rope of length 2 is: {num_places_visited}");

    // * Part Two
    // Determine the number of unique places the tail of the rope of length 10 visited in the input from the question
    let num_places_visited = part_two(question_rope_motions);
    println!("Part Two:\n  The number of unique locations the tail visited for the rope of length 10 is: {num_places_visited}");

    // Pause and wait for input from the user
    println!("\nDo you want to see the rope motions animated?");
    pause();
    // TODO: prompt how long the rope should be

    // Animate it!
    animate_rope_movements(question_rope_motions);
}

// region: Helpers

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
struct Position {
    x: i32,
    y: i32,
}

impl ops::Add for Position {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Position {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl ops::Sub for Position {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Position {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl ops::AddAssign for Position {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

/// Parse the question input into a vector of directions for each step in series of rope motions
fn parse_rope_motions(rope_motions: &str) -> Vec<Position> {
    rope_motions
        .split('\n')
        .map(|line| {
            let (direction, num_steps) = line
                .split_once(' ')
                .expect("Expected space in line - bad input format");

            // Get the direction depending on the character
            let direction = match direction {
                "U" => Position { x: 0, y: 1 },
                "D" => Position { x: 0, y: -1 },
                "L" => Position { x: -1, y: 0 },
                "R" => Position { x: 1, y: 0 },
                _ => panic!("Invalid direction in input '{direction}' - bad input"),
            };

            // Get the number of times head is to move that direction
            let num_steps = num_steps.parse::<usize>().expect(&format!(
                "Invalid number of repeats in input '{num_steps}'- bad input"
            ));

            // Return num_steps movements of this direction
            iter::repeat(direction).take(num_steps)
        })
        .flatten()
        .collect::<Vec<_>>()
}

/// Simulate the movement of a rope of the given length following the given `rope_motion_steps`, returning the set of positions the tail visited throughout the course of it
fn simulate_rope_movement(rope_motion_steps: Vec<Position>, rope_len: usize, mut animate_record: Option<&mut Vec<Vec<Position>>>) -> HashSet<Position> {
    // Create a set to hold the uniqe postitions that the tail has visited
    let mut tail_positions_visited = HashSet::<Position>::new();

    // Initialize the starting positions of the head and remaining trail
    let mut head_pos = Position { x: 0, y: 0 };
    let mut trail_positions = vec![
        Position { x: 0, y: 0 };
        rope_len
            .checked_sub(1)
            .expect("Rope needs to be at least 2 long")
    ];

    // Add the starting tail position to the places visited as the series starts off by "visiting" there
    tail_positions_visited.insert(
        *trail_positions
            .last()
            .expect("Rope needs to be at least 2 long"),
    );

    // Move the head along its list of steps and update the whole trail for each
    for movement_step in rope_motion_steps {
        // Move the head based on the movement_step
        head_pos += movement_step;

        // Update all elements of the trail based on their position to the one in front of them, returning the position if the final element
        let tail_position =
            trail_positions
                .iter_mut()
                .fold(head_pos, |relative_head, relative_tail| {
                    // ? Optionally could validate that the movement_step is a valid unit direction

                    // Update the relative tail based on its position to its relative head
                    let difference = relative_head - *relative_tail;
                    // If too far, update position of the relative_tail
                    if difference.x.abs() > 1 || difference.y.abs() > 1 {
                        // Get the change that relative_tail needs to do and update it
                        *relative_tail += Position {
                            x: difference.x.signum(),
                            y: difference.y.signum(),
                        };
                    }

                    // Return the new position for relative_tail to be used by the next elem
                    *relative_tail
                });

        // Record the position of the tail (in a set to effectively filter out duplicates)
        tail_positions_visited.insert(tail_position);

        // If animate_record provided, record the positions so they can be animated after
        if let Some(animate_record) = animate_record.as_mut() {
            animate_record.push([vec![head_pos], trail_positions.clone()].concat())
        }
    }

    // Return the tail_positions_visited upon completing simulating all the steps
    tail_positions_visited
}

// endregion

// region: Part One

#[test]
fn part_one_example_test() {
    // Read in the example rope motions trimming any surrounding whitespace
    let example_rope_motions = include_str!("../inputs/example").trim();

    // Determine the number of unique places the tail of the rope of length 2 visited in the example input from the question
    let num_places_visited = part_one(example_rope_motions);

    // Check if the example yields the same result as the question describes
    assert_eq!(num_places_visited, 13);
}

/// Simulate the movement of a rope of length 2 and find the number of unique places that the tail reached
fn part_one(rope_motions: &str) -> usize {
    // Get the list of steps to make and the directions to go in each
    let rope_motion_steps = parse_rope_motions(rope_motions);

    // Determine the number of unique places the tail of the rope of length 2 visited in the example input from the question
    let tail_positions_visited = simulate_rope_movement(rope_motion_steps, 2, None);

    // Return the number of unique places that the tail visited
    tail_positions_visited.len()
}

// endregion

// region: Part Two

#[test]
fn part_two_example_test() {
    // Read in the example rope motions trimming any surrounding whitespace
    let example_rope_motions = include_str!("../inputs/example").trim();

    // Determine the number of unique places the tail of the rope of length 10 visited in the example input from the question
    let num_places_visited = part_two(example_rope_motions);

    // Check if the example yields the same result as the question describes
    assert_eq!(num_places_visited, 1);
}

#[test]
fn part_two_larger_example_test() {
    // Check if the larger example yields the same result as the question describes
    assert_eq!(
        part_two(include_str!("../inputs/larger_example").trim()),
        36
    );
}

/// Simulate the movement of a rope of length 10 and find the number of unique places that the tail reached
fn part_two(rope_motions: &str) -> usize {
    // Get the list of steps to make and the directions to go in each
    let rope_motion_steps = parse_rope_motions(rope_motions);

    // Determine the number of unique places the tail of the rope of length 10 visited in the example input from the question
    let tail_positions_visited = simulate_rope_movement(rope_motion_steps, 10, None);

    // Return the number of unique places that the tail visited
    tail_positions_visited.len()
}

// endregion

// region: Rope Animation

/// Pause the program waiting for user input before continuing
fn pause() {
    // Wait for input...
    stdin().read(&mut [0]).unwrap();
}

#[test]
fn animate_rope_test() {
    animate_rope_movements(include_str!("../inputs/larger_example").trim());
}

/// Simulate the movement of a rope so that we can animate it!
fn animate_rope_movements(rope_motions: &str) {
    // Get the list of steps to make and the directions to go in each
    let rope_motion_steps = parse_rope_motions(rope_motions);

    // Vector to keep track of positions so we can animate it
    let mut rope_positions = Vec::<Vec<Position>>::new();

    // Determine the number of unique places the tail of the rope of length 10 visited in the example input from the question
    simulate_rope_movement(rope_motion_steps, 20, Some(&mut rope_positions));

    // Animate the rope
    animate_rope_positions(rope_positions);
}

/// Render the positions determined from simulating rope movements to the screen in a fun way
fn animate_rope_positions(rope_positions: Vec<Vec<Position>>) {
    // Find the max and min values of both x and y to figure out the dimensions of the entire screen we should render
    let min_x = rope_positions.iter().flatten().min_by_key(|pos| pos.x).unwrap().x;
    let max_x = rope_positions.iter().flatten().max_by_key(|pos| pos.x).unwrap().x;
    let min_y = rope_positions.iter().flatten().min_by_key(|pos| pos.y).unwrap().y;
    let max_y = rope_positions.iter().flatten().max_by_key(|pos| pos.y).unwrap().y;

    let width = (max_x - min_x) as usize + 1;
    let height = (max_y - min_y) as usize + 1;

    let base = vec![vec!['.'; width]; height];

    // Extra animation info
    let mut water_chunk = "~=~-".to_string();
    let fish = "><^>".to_string();
    let mut fish_pos = -6;
    let mut frame = 0;

    // Loop through each step of the motion list
    for step_positions in rope_positions {
        // Clear the screen
        // print!("{}[2J", 27 as char);
        print!("\x1B[2J\x1B[1;1H");

        // Construct the new 2d array
        let mut rope_display = base.clone();
        for (i, knot_position) in step_positions.iter().enumerate() {
            // Determine which character to display for this knot
            let display_char = match i {
                0 => 'H',
                1..=9 => i.to_string().chars().next().unwrap(),
                _ => '#',
            };

            // Update the character at the knot's position  
            rope_display[(knot_position.y - min_y) as usize][(knot_position.x - min_x) as usize] = display_char;
        }
        // Print the new 2d array
        println!("+{}+", vec!['-'; width].iter().collect::<String>());
        for row in rope_display.iter().rev() {
            println!("|{:}|", row.iter().collect::<String>());
        }

        // * Animate bottom water line
        let water = get_water_line(width, frame, &mut water_chunk, &fish, &mut fish_pos);
        // Print the water line
        println!("|{}|", water);
        frame += 1;

        println!("+{}+", vec!['-'; width].iter().collect::<String>());

        // Sleep for a certian amount of time
        thread::sleep(Duration::from_secs_f32(0.04));
    }
}

/// Animate the water line at the bottom with some moving water and a fish
fn get_water_line(width: usize, frame: i32, water_chunk: &mut String, fish: &str, fish_pos: &mut i32) -> String {
    // Create base water string using the current version of a water chunk
    let mut water = repeat(water_chunk.chars()).flatten().take(width).collect::<String>();

    // Cycle the water chunk every other frame so that it will look like it's moving next time
    if frame % 3 == 0 {
        let water_char = water_chunk.remove(0);
        water_chunk.push(water_char);
    }

    // * Add the visible part of the fish to the water at its position
    replace_water_with_visible_slice_of_object(&mut water, fish, *fish_pos);

    // Update fish position every third frame
    if frame % 3 == 0 {
        *fish_pos += 1;
    }

    // Add rocks with splash?
    // let mut rock = r"/\".to_string();
    // // Add splash to the rocks
    // // rock = rock + match (frame / 2) % 3 {
    // //     0 => "n",
    // //     1 => "a",
    // //     2 => "c",
    // //     _ => panic!("How can we get something else from a mod 3")
    // // };
    // replace_water_with_visible_slice_of_object(&mut water, &rock, 10);

    // Return the newly constructed water line
    water
}

/// Modifies that water String by overlaying the visible part of the object to the water String
fn replace_water_with_visible_slice_of_object(water: &mut String, object: &str, object_pos: i32) {
    // Example fish I used to work out this logic
    
    //    -4   0 2 4
    //     | | | | |
    //      ><^>

    // Fish pos: -3
    // Fish_full_range: -3..1
    // fish_full_range overlap with 0..width is just the last character -> 0..1
    // The part of the fish that we place in 0..1 is then (0 - fish_pos)..(1 - fish_pos) = 3..4 

    // Get the full range that the object would take up
    let object_full_range = object_pos..(object_pos + object.len() as i32);

    // Determine the overlap of that range with the string and render the fish if it is visible
    if let Some(overlap_range) = get_overlap_range(object_full_range, 0..(water.len() as i32)) {
        // 
        let fish_visible_slice_min = usize::try_from(overlap_range.start - object_pos).unwrap();
        let fish_visible_slice_max = usize::try_from(overlap_range.end - object_pos).unwrap();

        // Add the fish to the water
        water.replace_range(
            usize::try_from(overlap_range.start).unwrap()..usize::try_from(overlap_range.end).unwrap(),
            &object[fish_visible_slice_min..fish_visible_slice_max]);
    };
}

/// Returns the option of the overlap of two ranges, being None is they don't overlap
fn get_overlap_range(range_1: Range<i32>, range_2: Range<i32>) -> Option<Range<i32>> {
    // Get what the overlap range would be
    let start_max = range_1.start.max(range_2.start);
    let end_min = range_1.end.min(range_2.end);

    // Check if they overlap at all
    if start_max <= end_min {
        // Return the overlap range
        Some(start_max..end_min)
    }
    else {
        None
    }
}

// endregion
