use std::{iter, ops, thread, time::Duration};

fn main() {
    // Read in the cave structure from the question trimming any surrounding whitespace
    let question_cave_structure = include_str!("../inputs/question").trim();

    // * Part One
    // Simulate sand falling into the cave and count the number of particles of sand that pile up in it in the input from the question
    let total_sand_count = part_one(question_cave_structure);
    println!("Part One:\n  The total amount of sand that piled up in the cave before falling into the abyss was: {total_sand_count}");

    // * Part Two
    // Simulate the number of sand pieces it takes to fill up a cave with a floor described in the input from the question
    let total_sand_count = part_two(question_cave_structure);
    println!("Part Two:\n  The total amount of sand that it took to fill up the cave with an extensive floor was: {total_sand_count}");
}

// region: Cave Types

#[derive(Clone, Copy, PartialEq, Eq)]
/// A basic coordinate with some convenient operators - really this should be called vec2 or something like that
struct Coord {
    x: i32,
    y: i32,
}

impl ops::Add for Coord {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Coord {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl ops::Sub for Coord {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Coord {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl ops::AddAssign for Coord {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

struct CaveMap {
    map: Vec<Vec<char>>,
    coordinate_offset: Coord,
    // TODO: possibly add a render border that would show in rendering the cave, but not in the actual size of it
    // This would be cool to add to either side so we could render the sand that falls
    // These would be useful for animating if we wanted to separate the parts of the too-tall cave map...
    // height: usize,
    // width: usize,
}

/// Basically the Index & IndexMut traits but combining them and returning a Result of the reference instead of the reference itself
// ? Possible future, it might be better to just look into overriding the traits for get/get_mut...😅
trait IndexResult<Idx> {
    type OkType;
    type ErrType;
    fn index(&self, index: Idx) -> Result<&Self::OkType, Self::ErrType>;
    fn index_mut(&mut self, index: Idx) -> Result<&mut Self::OkType, Self::ErrType>;
}

impl IndexResult<Coord> for CaveMap {
    type OkType = char;
    // There's only going to be one way that this will error so unit is fine for the error type
    type ErrType = ();

    /// Return a reference to the right element of the map considering the coordinate offset given a world-space coordinate
    fn index(&self, index: Coord) -> Result<&Self::OkType, Self::ErrType> {
        // Get the local coodinate inside the map given the world space (global) coordinate index
        let local_coord = index - self.coordinate_offset;

        // Get the usize equivalents of the index, returning an Error if negative (would be out of bounds)
        let local_x = TryInto::<usize>::try_into(local_coord.x).map_err(|_| ())?;
        let local_y = TryInto::<usize>::try_into(local_coord.y).map_err(|_| ())?;

        // Return a reference to the appropriate element of the map
        self.map
            .get(local_y).ok_or(())?
            .get(local_x).ok_or(())
    }

    /// Return a mutable reference to the right element of the map considering the coordinate offset given a world-space coordinate
    fn index_mut(&mut self, index: Coord) -> Result<&mut Self::OkType, Self::ErrType> {
        // Get the local coodinate inside the map given the world space (global) coordinate index
        let local_coord = index - self.coordinate_offset;

        // Get the usize equivalents of the index, returning an Error if negative (would be out of bounds)
        let local_x = TryInto::<usize>::try_into(local_coord.x).map_err(|_| ())?;
        let local_y = TryInto::<usize>::try_into(local_coord.y).map_err(|_| ())?;

        // Return a mutable reference to the appropriate element of the map
        self.map
            .get_mut(local_y).ok_or(())?
            .get_mut(local_x).ok_or(())
    }
}

// * Implementing the Index + IndexMut operators directly didn't work as we wanted to have it return a Result, which you can't do

// endregion

// region: Helpers

/// Parse the cave structure into a vector of rock paths (represented by a vector of the coordinates of each vertex in the path)
fn parse_cave_structure(cave_structure: &str) -> Vec<Vec<Coord>> {
    cave_structure
        .split('\n')
        .map(|path| {
            path.split(" -> ")
                .map(|coord| {
                    // Read the coordinates str
                    let (x, y) = coord
                        .split_once(',')
                        .expect("Invalid cave coordinate - bad input");

                    // Parse x and y into coordinate numbers
                    Coord {
                        x: x.parse::<i32>().expect("Missing coordinate number x"),
                        y: y.parse::<i32>().expect("Missing coordinate number y"),
                    }
                })
                .collect()
        })
        .collect()
}

/// Generate the cave map from the `rock_structures` and return it (with its position offests in the x and y recorded)
fn generate_cave_map(rock_structures: Vec<Vec<Coord>>, sand_source: Coord) -> CaveMap {
    // Get max and min x and y among all the rocks and the sand source to determine the size of the cave map
    let all_rocks_iter = rock_structures.iter().flat_map(|path| path.iter());
    let max_x = all_rocks_iter
        .clone()
        .max_by_key(|coord| coord.x)
        .expect("Failed to find max x")
        .x
        .max(sand_source.x);
    let min_x = all_rocks_iter
        .clone()
        .min_by_key(|coord| coord.x)
        .expect("Failed to find min x")
        .x
        .min(sand_source.x);
    let max_y = all_rocks_iter
        .clone()
        .max_by_key(|coord| coord.y)
        .expect("Failed to find max y")
        .y
        .max(sand_source.y);
    let min_y = all_rocks_iter
        .clone()
        .min_by_key(|coord| coord.y)
        .expect("Failed to find min y")
        .y
        .min(sand_source.y);

    // Determine the height and width of the cave map
    let height = 1 + TryInto::<usize>::try_into(max_y - min_y)
        .expect("Invalid height - min y was bigger than max y");
    let width = 1 + TryInto::<usize>::try_into(max_x - min_x)
        .expect("Invalid width - min x was bigger than max x");

    // Create the cave map, initially filling it with air
    let mut cave_map = CaveMap {
        map: vec![vec!['.'; width]; height],
        coordinate_offset: Coord { x: min_x, y: min_y },
    };

    // Add the sand source to the cave map
    *cave_map.index_mut(sand_source).unwrap() = '+';

    // Add each of the rock path structures to the cave map
    for rock_path in rock_structures {
        let rock_path_final_coord = rock_path
            .iter()
            .reduce(|prev_coord, current_coord| {
                // Note: for now, we are going to assume the input is good and that we won't be creating any infinite iters haha

                // Get the direction of the rock line starting at the prev_coord and ending at current_coord
                let rock_line_direction = Coord {
                    x: (current_coord.x - prev_coord.x).signum(),
                    y: (current_coord.y - prev_coord.y).signum(),
                };

                // Create an iter that can traverse the rock line from prev to current
                let mut position = *prev_coord;
                let rock_line_iter = iter::from_fn(|| {
                    // Check if we are done with the line segment - if the last position was the end position of the line, be done
                    if position == *current_coord { return None; }

                    // Record the position to return with the iter (we want it to iterate over the first coord so we will record this before incrementing)
                    let iter_position = position;

                    // Increment position by direction for the next iter to use
                    position += rock_line_direction;

                    // Return the iter posiion
                    Some(iter_position)
                });

                // Fill each of the positions in the line between the two points with rock, aka '#'
                for rock_coord in rock_line_iter {
                    *cave_map.index_mut(rock_coord).unwrap() = '#';
                }

                // Pass the current_coord as the prev_coord for the next elem
                current_coord
            })
            .expect(
                "Failed to traverse the whole (or possibly just the last part of) the rock path",
            );

        // Fill in the last element of the reduced rock path with rock, aka '#'
        *cave_map.index_mut(*rock_path_final_coord).unwrap() = '#';
    }

    // Return the cave map
    cave_map
}

/// Simulates a a sand partical falling, returning the coordinate of the final position it came to a rest or an error if it fell into the abyss
fn simulate_sand_particle_falling(cave_map: &CaveMap, sand_source: Coord) -> Result<Coord, ()> {
    // Spawn a unit of sand at the sand source
    let mut particle_position = sand_source;

    // Define the direction checks that the sand physics follow
    let direction_checks = [(0, 1), (-1, 1), (1, 1)]
        .into_iter()
        .map(|(x, y)| Coord { x, y })
        .collect::<Vec<_>>();

    // Create an iterator that returns positions of a unit of sand until it comes to rest
    // This returns a result of the position that will be an err if the position is outside the cave map
    let sand_movement_iter = iter::from_fn(|| {
        // Get the next position this sand particle should go, bubbling up None (with ?) if there wasn't one
        direction_checks.iter().find_map(|direction| {
            // Find the first direction in which the poisition is open (either in the map or not)
            // A better way to describe this is to find the first one that doesn't return an Ok(char) where char is '#' or 'o' - Ok()
            match cave_map.index(particle_position + *direction) {
                Ok('#' | 'o') => None,
                open_space_result => {
                    // Update the particle position in the selected direction
                    particle_position += *direction;

                    // Return the new position
                    Some(open_space_result.map(|_| particle_position))
                }
            }
        })
    });

    // Iterate through our sand movement iterator to simulate the sand falling.
    // The iterator will stop once the sand has come to rest, and it will return an error if the open space it is trying to move to is outside the cave map
    for next_position in sand_movement_iter {
        match next_position {
            Ok(_next_position) => {
                // ? Note: We could animate the next poisition here
                continue;
            },
            // If sand goes off the edge, the cave structure is full of sand (just like professor Zei's life's ambition), so we can return the number of units of sand that have come to a rest
            Err(_) => return Err(()),
        }
    }

    // Return the final position
    Ok(particle_position)
}

fn render_cave(cave_map: &CaveMap) -> String {
    cave_map
        .map
        .iter()
        .map(|row| row.iter().collect::<String>())
        .collect::<Vec<_>>()
        .join("\n")
}

fn animate_cave_map(cave_map: &CaveMap) {
    // Move the cursor to the start of the screen
    print!("\x1B[1;1H");

    // Print the cave map
    println!("{}", render_cave(cave_map));

    // Sleep for a certain amount of time to create a visible framerate in the animation
    thread::sleep(Duration::from_secs_f32(0.02));
}

// endregion

// region: Part One

#[test]
fn part_one_example_test() {
    // Read in the example cave structure trimming any surrounding whitespace
    let example_cave_structure = include_str!("../inputs/example").trim();

    // Simulate sand falling into the cave and count the number of particles of sand that pile up in it in the example input from the question
    let total_sand_count = part_one(example_cave_structure);

    // Check if the example yields the same result as the question describes
    assert_eq!(total_sand_count, 24);
}

/// Generate a cave map and simulate sand falling into it, counting the number of units of sands that it takes to fill it up
fn part_one(cave_structure: &str) -> u32 {
    // Parse the cave_structure input into a vector of rock paths
    let rock_structures = parse_cave_structure(cave_structure);

    // Define the coordinate of the sand source of where sand is spilling in from
    let sand_source = Coord { x: 500, y: 0 };

    // Generate the cave map given the rock structure and the location of the sand source
    let mut cave_map = generate_cave_map(rock_structures, sand_source);

    // println!("The cave_map looks like:\n{}", render_cave(&cave_map));

    // * Animation
    // Clear the screen
    // print!("\x1B[2J\x1B[1;1H");

    // Starting simulating sand falling
    let mut resting_sand_count = 0;
    loop {
        // Simulate the particle falling and retrieve its final position
        let Ok(sand_position) = simulate_sand_particle_falling(&cave_map, sand_source) else {
            // If sand has begun to fall into the abyss, break
            break;
        };

        // If sand comes to a stop, increment the count of units of sand that have come to a rest and place the sand in its final location
        resting_sand_count += 1;
        *cave_map
            .index_mut(sand_position)
            .expect("Final sand position is invalid") = 'o';

        // * Animate: print the cave_map
        // animate_cave_map(&cave_map);
    }

    // Return the count of the grains of sand that have come to rest
    resting_sand_count
}

// endregion

// region: Part Two

#[test]
fn part_two_example_test() {
    // Read in the example cave structure trimming any surrounding whitespace
    let example_cave_structure = include_str!("../inputs/example").trim();

    // Simulate the number of sand pieces it takes to fill up a cave with a floor described in the example input from the question
    let total_sand_count = part_two(example_cave_structure);

    // Check if the example yields the same result as the question describes
    assert_eq!(total_sand_count, 93);
}

/// Generate a cave map with a floor and return the number pieces of sand it takes to fill up the location where the sand is pouring in
fn part_two(cave_structure: &str) -> u32 {
    // Parse the cave_structure input into a vector of rock paths
    let mut rock_structures = parse_cave_structure(cave_structure);

    // Define the coordinate of the sand source of where sand is spilling in from
    let sand_source = Coord { x: 500, y: 0 };

    // First find the initial max y of the rock structures
    let max_y = rock_structures
        .iter()
        .flat_map(|path| {
            path.iter().map(|coord| coord.y)
        })
        .max()
        .expect("Failed to find max y")
        .max(sand_source.y);

    // Add 2 to the max y for y actual max y -> the y position of the floor
    let floor_y = max_y + 2;

    // Given the y of the floor and the sand source, determine the vertices of the sized rock path needed for the floor of the cave to hold the max amount of sand
    let required_outward_width = floor_y - sand_source.y;
    let floor_rock_path = vec![
        Coord {
            x: sand_source.x - required_outward_width,
            y: floor_y,
        },
        Coord {
            x: sand_source.x + required_outward_width,
            y: floor_y,
        },
    ];

    // Add the rock path for the floor to the rock structures from the original cave scan
    rock_structures.push(floor_rock_path);

    // Generate the cave map given the rock structures and the location of the sand source
    let mut cave_map = generate_cave_map(rock_structures, sand_source);

    // Render the cave map
    // println!("The cave_map looks like:\n{}", render_cave(&cave_map));

    // * Animation
    // Clear the screen
    // print!("\x1B[2J\x1B[1;1H");

    // Simulate sand falling again, but with a new end condition...
    let mut resting_sand_count = 0;
    loop {
        // Simulate the particle falling and retrieve its final position
        let Ok(sand_position) = simulate_sand_particle_falling(&cave_map, sand_source) else {
            panic!("There shouldn't be any sand particles that fall outside the cave in part two...");
        };

        // If sand comes to a stop, increment the count of units of sand that have come to a rest and place the sand in its final location
        resting_sand_count += 1;
        *cave_map
            .index_mut(sand_position)
            .expect("Final sand position is invalid") = 'o';

        // If this particle of sand ends up in the sand source position, the source is now blocked, and we are done
        if sand_position == sand_source {
            break;
        }

        // * Animate: print the cave_map
        // animate_cave_map(&cave_map);
    }

    // Return the count of the grains of sand that have come to rest
    resting_sand_count
}

// endregion
