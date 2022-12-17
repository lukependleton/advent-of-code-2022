use coordinate::Coord;
use regex::Regex;

pub mod coordinate;
pub mod worldmap;

// region: Helpers

/// Parses the sensor data input into a vector of pairs of sensor and beacon coordinates + the manhattan distance between them
pub fn parse_sensor_data(sensor_data: &str) -> Vec<(Coord, Coord, u32)> {
    sensor_data
        .split('\n')
        .map(|line| {
            let re = Regex::new(
                "Sensor at x=(?P<sensor_x>-?[0-9]+), y=(?P<sensor_y>-?[0-9]+): \
                closest beacon is at x=(?P<beacon_x>-?[0-9]+), y=(?P<beacon_y>-?[0-9]+)",
            )
            .expect("Bad regex defined");

            // Perform the regex match on the sensor data in the given line
            let caps = re.captures(line).unwrap();

            // Get the coordinates for the sensor
            let sensor_coord = Coord {
                x: caps
                    .name("sensor_x")
                    .expect("Failed to match the sensor_x group")
                    .as_str()
                    .parse::<i32>()
                    .expect("Sensor x value is an invalid number"),
                y: caps
                    .name("sensor_y")
                    .expect("Failed to match the sensor_y group")
                    .as_str()
                    .parse::<i32>()
                    .expect("Sensor y value is an invalid number"),
            };

            // Get the coordinates for the beacon
            let beacon_coord = Coord {
                x: caps
                    .name("beacon_x")
                    .expect("Failed to match the beacon_x group")
                    .as_str()
                    .parse::<i32>()
                    .expect("Beacon x value is an invalid number"),
                y: caps
                    .name("beacon_y")
                    .expect("Failed to match the beacon_y group")
                    .as_str()
                    .parse::<i32>()
                    .expect("Beacon y value is an invalid number"),
            };

            // Return the pair of sensor and beacon coordinates and the manhattan distance between them
            (
                sensor_coord,
                beacon_coord,
                sensor_coord.manhattan_distance(&beacon_coord),
            )
        })
        .collect()
}

// endregion
