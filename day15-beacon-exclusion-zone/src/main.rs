use ::day15_beacon_exclusion_zone::{
    coordinate::Coord,
    parse_sensor_data,
    worldmap::{IndexResult, WorldMap},
};

fn main() {
    // Read in the sensor data from the question trimming any surrounding whitespace
    let question_sensor_data = include_str!("../inputs/question").trim();

    // * Part One
    // Find the number of positions in the given row that are not beacons using the question's sensor data input
    let part_one_result = part_one(question_sensor_data, 2000000);
    println!("Part One:\n  The number of positions the beacon can't be in row 2000000 is: {part_one_result}");

    // * Part Two
    // Find the tuning frequency of the distress beacon within the 4000000x4000000 block using the question's sensor data input to elimate the places it can't be
    let tuning_frequency = part_two(question_sensor_data, 4000000);
    println!("Part Two:\n  The tuning frequency of the distress beacon was determined to be: {tuning_frequency}");
}

// region: Part One

#[test]
fn part_one_example_test() {
    // Read in the example sensor data trimming any surrounding whitespace
    let example_sensor_data = include_str!("../inputs/example").trim();

    // Find the number of positions in the given row that are not beacons using the example's sensor data input
    let beacon_less_positions = part_one(example_sensor_data, 10);

    // Check if the example yields the same result as the question describes
    assert_eq!(beacon_less_positions, 26);
}

/// Find the number of places that are not beacons in the given y row using the given sensor data
fn part_one(sensor_data: &str, y_in_question: i32) -> usize {
    // Parse the input into a vector of sensor-beacon pairs and their manhattan distance
    let sensor_data = parse_sensor_data(sensor_data);

    // Find the sensors whose range overlaps with the y in question as the others won't be needed to be checked for this row
    let sensor_data = sensor_data
        .into_iter()
        .filter(|(sensor_coord, _, distance)| sensor_coord.y.abs_diff(y_in_question) <= *distance)
        .collect::<Vec<_>>();

    let sensor_map = generate_world_map_row(&sensor_data, y_in_question);
    // println!("Sensor map:\n{}", sensor_map.render());

    // println!("Filled in row was:\n{:?}", row_in_question.iter().collect::<String>());

    // Count the number of '#' chars in the row in question to get the number of positions where a beacon cannot be present
    sensor_map
        .get_row(y_in_question)
        .unwrap()
        .iter()
        .filter(|character| **character == '#')
        .count()
}

/// Create the world map but only row of the y in question for efficiency, and fill it with the relevant sensor data
fn generate_world_map_row(sensor_data: &Vec<(Coord, Coord, u32)>, y_in_question: i32) -> WorldMap {
    // println!("Generating sensor map of the row {y_in_question} with the sensor data...");

    // Determine the x ranges that we are working with for these sensors & beacons
    let max_x = sensor_data
        .iter()
        .map(|(sensor_coord, _, distance)| sensor_coord.x + *distance as i32)
        .max()
        .expect("Failed to find max x");
    let min_x = sensor_data
        .iter()
        .map(|(sensor_coord, _, distance)| sensor_coord.x - *distance as i32)
        .min()
        .expect("Failed to find min x");
    let width = 1 + TryInto::<usize>::try_into(max_x - min_x)
        .expect("Invalid width - min x was bigger than max x");

    // Construct a worldmap of only the row for the y in question
    let mut row_in_question = WorldMap::new(
        1,
        dbg!(width),
        '.',
        Coord {
            x: min_x,
            y: y_in_question,
        },
    );

    // Loop through the row in question, for each filling in sensor, beacon, and beacon-blocked tiles
    for x in min_x..max_x {
        let coord = Coord {
            x,
            y: y_in_question,
        };

        // Loop through the sensors and fill in the appropriate data for the current cell
        for (sensor_coord, beacon_coord, distance) in sensor_data {
            // Check if this is a senor position
            if *sensor_coord == coord {
                // Add the sensor
                *row_in_question.index_mut(coord).unwrap() = 'S';
                break;
            }

            // Check if this is a beacon position
            if *beacon_coord == coord {
                // Add the beacon
                *row_in_question.index_mut(coord).unwrap() = 'B';
                break;
            }

            // Check if this is a position that this sensor's aoe diamond blocks beacons from being
            if coord.manhattan_distance(sensor_coord) <= *distance {
                // Add that this cell can't contain a beacon
                *row_in_question.index_mut(coord).unwrap() = '#';
                break;
            }
        }
    }

    // Return the constructed row in question
    row_in_question
}

// endregion

// region: Part Two

#[test]
fn part_two_example_test() {
    // Read in the example sensor data trimming any surrounding whitespace
    let example_sensor_data = include_str!("../inputs/example").trim();

    // Find the tuning frequency of the distress beacon within the 4000000x4000000 block using the example's sensor data input to elimate the places it can't be
    println!("Testing the example sensors for the distress beacon in the x & y range (0, 20)");
    let tuning_frequency = part_two(example_sensor_data, 20);

    // Check if the example yields the same result as the question describes
    assert_eq!(tuning_frequency, 56000011);
}

/// Calculate the tuning frequncy of the distress beacon upon finding its coordinate using the sensor
/// data to work out the places that it isn't in the given `max_distane` square
fn part_two(sensor_data: &str, max_distance: i32) -> u64 {
    // Parse the input into a vector of sensor-beacon pairs and their manhattan distance
    let sensor_data = parse_sensor_data(sensor_data);

    // Loop through the "max_distance square", skipping across sensors' diamond shaped areas as you encounter
    //  them until you find the coordinate not inside any of the sensors' ranges, aka the distress beacon
    let mut distress_coord = None;
    'all_rows: for j in 0..max_distance {
        let mut i = 0;
        'inside_row: while i < max_distance {
            // Define the coord for this (i, j)
            let coord = Coord { x: i, y: j };

            // Loop through the sensors and check if this coord is in their range
            for (sensor_coord, _, range) in sensor_data.iter() {
                // Check if this square is in the sensor's range
                let dist_to_sensor = coord.manhattan_distance(sensor_coord);
                if dist_to_sensor <= *range {
                    // If so, determine the width of this sensor diamond (that is left to traverse) and jump to the end of it

                    // The difference from the sensor's y tells us how far we are from the vertical center of the diamond (how wide)
                    let y_distance_from_diamond_center = coord.y.abs_diff(sensor_coord.y) as i32;

                    // Get the width of the diamond for this row considering the range of the sensor and the vertical distance we are from the center
                    let diamond_width = 1 + (*range as i32 - y_distance_from_diamond_center) * 2;

                    // Determine how far we are at from the start of the diamond
                    let x_dist_from_beginning_of_diamond =
                        coord.x - (sensor_coord.x - *range as i32 + y_distance_from_diamond_center);

                    // Jump to the end of the sensor's range
                    let dist_to_jump = diamond_width - x_dist_from_beginning_of_diamond;
                    i += dist_to_jump;
                    continue 'inside_row;
                }
            }

            // If the current coordinate is not within any of the sensor ranges, this is the distress coord!
            println!("Found the distress coord! It's {coord:?}");
            distress_coord = Some(coord);
            break 'all_rows;
        }
    }

    // Extract the coordinate
    let distress_coord = distress_coord.expect("Failed to find the coordinate of the distress beacon");

    // Calculate the tuning frequency given the coordinate of the distress beacon
    distress_coord.x as u64 * 4000000 + distress_coord.y as u64
}

// endregion
