use std::{iter, thread, time::Duration};

use ::day15_beacon_exclusion_zone::{
    coordinate::Coord,
    parse_sensor_data,
    worldmap::{IndexResult, WorldMap},
};

fn main() {
    // Read in the example sensor data trimming any surrounding whitespace
    let example_sensor_data = include_str!("../../inputs/example").trim();

    // Parse the data
    let sensor_data = parse_sensor_data(example_sensor_data);

    // Generate the full sensor map, animating it!
    generate_world_map(&sensor_data, true);
}

fn generate_world_map(sensor_data: &Vec<(Coord, Coord, u32)>, animation: bool) -> WorldMap {
    println!("Getting min/maxes...");
    // Determine the x and y ranges that we are working with for these sensors & beacons
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
    let max_y = sensor_data
        .iter()
        .map(|(sensor_coord, _, distance)| sensor_coord.y + *distance as i32)
        .max()
        .expect("Failed to find max y");
    let min_y = sensor_data
        .iter()
        .map(|(sensor_coord, _, distance)| sensor_coord.y - *distance as i32)
        .min()
        .expect("Failed to find min y");
    println!("Got min/maxes!");

    // Determine the height and width of the map to hold of the sensor/beacon info
    let height = 1 + TryInto::<usize>::try_into(max_y - min_y)
        .expect("Invalid height - min y was bigger than max y");
    let width = 1 + TryInto::<usize>::try_into(max_x - min_x)
        .expect("Invalid width - min x was bigger than max x");

    println!("Height is: {height}, width is {width}");

    // Create the sensor map, initially filling it with air
    let mut sensor_map = WorldMap::new(height, width, '.', Coord { x: min_x, y: min_y });

    // * Animation
    if animation {
        // Clear the screen
        print!("\x1B[2J\x1B[1;1H");
    }

    // For each sensor, draw the sensor + beacon and fill in the sensor map with their ranges that they eliminate beacons in
    for (sensor_coord, beacon_coord, distance) in sensor_data {
        // println!("Processing sensor: {:?}", *sensor_coord);
        // Add the sensor
        *sensor_map.index_mut(*sensor_coord).unwrap() = 'S';

        // Add the beacon
        *sensor_map.index_mut(*beacon_coord).unwrap() = 'B';

        // Fill in the positions that the sensor diamond blocks beacons from being
        for radius in 1..(*distance + 1) {
            let diamond_iter = get_diamond(*sensor_coord, radius);

            // Could do some cool animations here...
            for position in diamond_iter {
                if *sensor_map.index(position).unwrap() == '.' {
                    *sensor_map.index_mut(position).unwrap() = '#';
                }
            }

            if animation {
                animate_sensor_map(&sensor_map);
            }
        }
    }

    // Return the generated sensor map
    sensor_map
}

fn get_diamond(center: Coord, radius: u32) -> impl Iterator<Item = Coord> {
    // Given the radius determine the number of steps to construct a side
    let num_steps_a_side = radius as usize;

    // Define the directions to follow to construct each side
    let side_directions = [(1, 1), (-1, 1), (-1, -1), (1, -1)]
        .into_iter()
        .map(|(x, y)| Coord { x, y })
        .collect::<Vec<_>>();

    // Loop through the diamond
    // Note: For the given order of side directions I selected, the start position will be at the top (visually) of the diamond
    side_directions
        .into_iter()
        .flat_map(move |direction| iter::repeat(direction).take(num_steps_a_side))
        .scan(
            center
                + Coord {
                    x: 0,
                    y: -1 * radius as i32,
                },
            |position, direction| {
                // Move the position in the direction
                *position += direction;

                // Return the new position
                Some(*position)
            },
        )
}

fn animate_sensor_map(sensor_map: &WorldMap) {
    // Move the cursor to the start of the screen
    print!("\x1B[1;1H");

    // Print the sensor map
    println!("{}", sensor_map.render(true));

    // Sleep for a certain amount of time to create a visible framerate in the animation
    thread::sleep(Duration::from_secs_f32(0.05));
}
