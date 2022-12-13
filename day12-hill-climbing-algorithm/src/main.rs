use std::{
    collections::{HashMap, VecDeque},
    thread,
    time::Duration,
};

use colored::Colorize;

fn main() {
    // Read in the heightmap from the question trimming any surrounding whitespace
    let question_heightmap = include_str!("../inputs/question_try2").trim();

    // * Part One
    // Find the length of the shortest path to the location with the best signal in the heightmap from the question
    let shortest_distance_to_best_signal = part_one(question_heightmap);
    println!("Part One:\n  The shortest distance it would take to get to the place with the best signal is: {shortest_distance_to_best_signal}");

    // * Part Two
    // Find the shortest distance from a starting lowest point to the highest point in the example input from the question
    let shortest_distance_to_best_signal = part_two(question_heightmap);
    println!("Part Two:\n  The shortest distance among the reachable lowest points to the highest point (best signal) is: {shortest_distance_to_best_signal}");
}

// region: Helpers

type Coordinate = (usize, usize);

/// Determine the height of the given lowercase alpha character (a-z)
fn alpha_height(alpha_character: char) -> u8 {
    // Get the ascii value of the given character
    let character_val = TryInto::<u8>::try_into(alpha_character).expect("Invalid character");
    // To get the height of a character, get its difference from a (ascii value of 97)
    // TODO: possibly don't assume that it is in the right range... (0..25)
    TryInto::<u8>::try_into(character_val - 97)
        .expect("Character needs to be from a-z for this method")
}

/// Parses the input heightmap str, returning a tuple of the the heightmap and the start/end positions
fn parse_heightmap(heightmap: &str) -> (Vec<Vec<u8>>, (Coordinate, Coordinate)) {
    let mut start = None;
    let mut end = None;
    let heightmap = heightmap
        .split('\n')
        .enumerate()
        .map(|(j, line)| {
            line.chars()
                .enumerate()
                .map(|(i, height_char)| match height_char {
                    'S' => {
                        // Record the start position
                        start = Some((i, j));
                        // Return 0 as the start position height
                        0
                    }
                    'E' => {
                        // Record the end position
                        end = Some((i, j));
                        // Return 25 as the end position height
                        25
                    }
                    height_char => alpha_height(height_char),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    // Return the heightmap with the starting and ending coordinates
    (
        heightmap,
        (
            start.expect("Couldn't find starting location in heightmap"),
            end.expect("Couldn't find ending location in heightmap"),
        ),
    )
}

/// Given a certain coordinate at (`x`, `y`), determine which steps among the four cardinal directions that can be traversed
/// given their respective heights considering whether we are traversing in reverse or not.
fn get_valid_steps(
    x: usize,
    y: usize,
    heightmap: &Vec<Vec<u8>>,
    map_height: usize,
    map_width: usize,
    reversed: bool,
) -> Vec<Coordinate> {
    // Get array of possible steps: up, down, left, right
    let directions = [(0, 1), (0, -1), (-1, 0), (1, 0)];

    // Iterate over steps and return ones that are valid
    directions
        .into_iter()
        .filter_map(|(step_x, step_y)| {
            // Get the usize coords of the next step, returning None if that position is invalid (negative pretty much)
            let stepped_coordinate = (
                TryInto::<usize>::try_into(x as i32 + step_x).ok()?,
                TryInto::<usize>::try_into(y as i32 + step_y).ok()?,
            );

            // Check if this is a valid coordinate on the map
            if (0..map_width).contains(&stepped_coordinate.0)
                && (0..map_height).contains(&stepped_coordinate.1)
            {
                let current_height = heightmap[y][x];
                let stepped_height = heightmap[stepped_coordinate.1][stepped_coordinate.0];

                // * Check if this is a valid place to step to height-wise - at most one heigher
                // ! Apparently, you can also jump down any amount of height - it is just higher that you can only move by 1
                // (current_height.abs_diff(stepped_height) <= 1).then(|| stepped_coordinate)

                // Get the differenve in height between the two
                let mut height_difference = stepped_height as i8 - current_height as i8;
                // If we are traversing in the opposite direction, we need to check the opposite directions by multiplying by -1
                if reversed {
                    height_difference *= -1;
                }

                // Check if the difference between the two is a valid step, returning Some(stepped_coordinate) if it is
                (height_difference <= 1).then(|| stepped_coordinate)
            } else {
                None
            }
        })
        .collect()
}

/// Does a breadth first seach traversal of the graph starting at `start_coordinate` to find the shortest distance to each coordinate that it can reach
fn breadth_first_search_shortest_distance_to_coordinates(
    heightmap_str: &str,
    heightmap: &Vec<Vec<u8>>,
    start_coordinate: Coordinate,
    reversed: bool,
    add_debug_animation: bool,
) -> HashMap<Coordinate, u32> {
    // Get the height and width of the heightmap
    let height = heightmap.len();
    let width = heightmap
        .first()
        .expect("Missing first line of the heightmap - invalid input")
        .len();

    // Need to find the shortest path...thinking through what that means
    // Maybe to start, we'll just brute force it, but might need dijkstra's algorithm (maybe just for weighted graphs though?) or some other shortest path one
    // Branch pruning might be good with branch and bound stuff...

    // Generate the connectedness graph for the heightmap
    // Create basically a "2D adjacency list" of vector of the directions that can be stepped to from each location to represent the unweighted graph of the problem
    let mut adjacency_graph = vec![vec![Vec::<Coordinate>::new(); width]; height];
    for j in 0..height {
        for i in 0..width {
            // Determine the valid steps for this location
            adjacency_graph[j][i] = get_valid_steps(i, j, &heightmap, height, width, reversed);
        }
    }

    // This one is effectively an unweighted graph - there's a good chance the difference in part two is that it will be weighted (possibly using heights as weights)
    // So...lets find the shortest path on an unweighted graph!
    // Since the question only asks for the length of the path, we don't need to record the path itself

    // Using BFS (which requires a queue) stating from the start position, lets find the minimum distances to each coordinate on the height map
    let mut distances = HashMap::<Coordinate, u32>::new();
    let mut bfs_vertices = VecDeque::<Coordinate>::new();

    // Initialize the queue with the starting vertx/node and set its distance to 0
    bfs_vertices.push_back(start_coordinate);
    distances.insert(start_coordinate, 0);

    // * Debug create a visual colored representation of the heightmap
    let mut colored_heightmap = if add_debug_animation {
        // Only do this (kind of) costly task if we need to
        construct_colored_heightmap(heightmap_str)
    } else {
        Vec::default()
    };
    let mut current_distance = 0;
    let mut current_distance_coords = Vec::<Coordinate>::new();

    while let Some(vertex) = bfs_vertices.pop_front() {
        // Get the valid directions/neighbors of this vertex
        let valid_neighbors = adjacency_graph[vertex.1][vertex.0].clone();
        for neighbor in valid_neighbors {
            // Check if we need to record the distance of this neighbor vertex. We would only want to if it were shorter, but since
            //  it is breadth first search and the distance we are at in each iteration never goes down, we really want to check if
            //  it is in the distances map yet
            if !distances.contains_key(&neighbor) {
                // We haven't seen this neighbor node yet

                // Record the distance of this neighbor node as the distance of the current node + 1
                let neighbor_distance = *distances.get(&vertex).unwrap() + 1;
                distances.insert(neighbor, neighbor_distance);

                // Add the neigbor to the queue
                bfs_vertices.push_back(neighbor);

                // * Debug print that we visited this vertex
                if add_debug_animation {
                    // If this is a new distance, the current distance is done, so print out all the coordinates from it
                    if neighbor_distance > current_distance {
                        // Set the new distance
                        current_distance = neighbor_distance;

                        // Print all of the coodinates from the now previous distance
                        debug_animate_heightmap_traversal(
                            &current_distance_coords,
                            &mut colored_heightmap,
                        );

                        // Flush the coordinates from the now previous_distance
                        current_distance_coords.clear();
                    }
                    // Add this neighbor to the list of coordinate for the current distance
                    current_distance_coords.push(neighbor);
                }
            }
        }
    }

    // Return the hashmap of coordinates and their cooresponding distances
    distances
}

// endregion

// region: Debug Animations

fn construct_colored_heightmap(heightmap_str: &str) -> Vec<Vec<String>> {
    heightmap_str
        .split('\n')
        .map(|line| {
            line.chars()
                .map(|character| character.to_string())
                .collect()
        })
        .collect()
}

fn debug_animate_heightmap_traversal(
    coords_of_this_level: &Vec<Coordinate>,
    colored_heightmap: &mut Vec<Vec<String>>,
) {
    // Clear the screen
    print!("\x1B[2J\x1B[1;1H");

    // Color the character (String) at each of the coordinates in this level to display it as visited
    for current_coord in coords_of_this_level {
        colored_heightmap[current_coord.1][current_coord.0] = colored_heightmap[current_coord.1][current_coord.0]
            .blue()
            .bold()
            .to_string();
    }

    // Print it
    let colored_grid_string = colored_heightmap
        .iter()
        .map(|row| row.join(""))
        .collect::<Vec<_>>()
        .join("\n");
    println!("{colored_grid_string}");

    // Sleep here for a framerate to the animation
    thread::sleep(Duration::from_secs_f32(0.05));
}

// endregion

// region: Part One

#[test]
fn part_one_example_test() {
    // Read in the example heightmap trimming any surrounding whitespace
    let example_heightmap = include_str!("../inputs/example").trim();

    // Find the length of the shortest path to the location with the best signal in the example heightmap from the question
    let shortest_distance_to_best_signal = part_one(example_heightmap);

    // Check if the example yields the same result as the question describes
    assert_eq!(shortest_distance_to_best_signal, 31);
}

/// Find the shortest distance that it takes to travel from the starting position to the end position of the heightmap
fn part_one(heightmap_str: &str) -> u32 {
    // Parse the heightmap input into the 2d vector of heights and the starting & ending positions of the problem
    let (heightmap, (start, end)) = parse_heightmap(heightmap_str);

    // Perform a breadth-first search to get the shortest distances of each reachable coordinate from the starting position
    let distances = breadth_first_search_shortest_distance_to_coordinates(
        heightmap_str,
        &heightmap,
        start,
        false,
        false,
    );

    // Return the distance to the shortest distance to the end coordinate in our heightmap
    *distances
        .get(&end)
        .expect("Failed to construct a path from the start to the end in our heightmap")
}

// endregion

// region: Part Two

#[test]
fn part_two_example_test() {
    // Read in the example heightmap trimming any surrounding whitespace
    let example_heightmap = include_str!("../inputs/example").trim();

    // Find the shortest distance from a starting lowest point to the highest point in the example input from the question
    let shortest_distance_to_best_signal = part_two(example_heightmap);

    // Check if the example yields the same result as the question describes
    assert_eq!(shortest_distance_to_best_signal, 29);
}

/// Find the shortest distance of any of the coordinates with the lowest height to the location with the best signal
fn part_two(heightmap_str: &str) -> u32 {
    // Parse the heightmap input into the 2d vector of heights and the starting & ending positions of the problem
    let (heightmap, (_, end)) = parse_heightmap(heightmap_str);

    // To find the distances to the end for each of the different starting points, instead of doing it individually for each of them,
    //  we can simply go the other direction and start from the end location. Then we can use that data of the distances of each
    //  location from the end to find which of the elements that have the minimum height 'a' in are closest.

    // Perform a breadth-first search to get the shortests distances of each reachable coordinate from the starting position
    let distances = breadth_first_search_shortest_distance_to_coordinates(
        heightmap_str,
        &heightmap,
        end,
        true,
        true,
    );

    // Get all of the coordinates that have the lowest height 'a'
    let lowest_height_coordinates = heightmap
        .iter()
        .enumerate()
        .flat_map(|(j, row)| {
            row.iter()
                .enumerate()
                .map(move |(i, height)| ((i, j), *height))
        })
        .filter_map(
            |((i, j), height)| {
                if height == 0 {
                    Some((i, j))
                } else {
                    None
                }
            },
        )
        .collect::<Vec<_>>();

    // Return the minimum distance of any of the coordinates with the lowest height
    *lowest_height_coordinates
        .iter()
        .filter_map(|coordinate| distances.get(coordinate))
        .min()
        .expect("Failed to find a minimum distance from a lowest coordinate to the end in our heightmap")
}

// endregion
