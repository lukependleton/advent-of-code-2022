// * A diagonal appraoch to the solution - it works, but is super confusing to think through
// Note: this file currently isn't tied in with the rest of the program - it's not compiled

// This one ended up taking 6.910 seconds to finish in the one test I did
// The normal iteration approach took 4.961 seconds... so that approach was bot was way less complicated and even faster haha


/// Calculate the tuning frequncy of the distress beacon upon finding its coordinate using the sensor
/// data to work out the places that it isn't in the given `max_distane` square
fn part_two(sensor_data: &str, max_distance: i32) -> u64 {
    // Parse the input into a vector of sensor-beacon pairs and there distance
    let sensor_data = parse_sensor_data(sensor_data);

    // println!("Sensor data:");
    // for sensor in sensor_data.iter() {
    //     println!("  {sensor:?}")
    // }

    // Thinking about how to avoid iterating over 4000000 * 4000000 places haha
    // The sensor diamonds each remove huge chunks of the possilbe areas at a time
    // We should be able to take advantage of that...
    // However, we can't easily take advantage of that because the info that they are giving us is in the wrong direction
    // That is, they are taking out large diagonal chunks of the map as opposed to the vertical and horizontal directions that we are iterating through and organizing our map by
    // However, there are effetively diagonal grids that we can think of as their own coordinates
    // Thinking more about it, we can iterate through variable length diagonal rows similar how the bishop moves in chess
    // Specifically, there are basically two types that diagonal rows can be a part of, which are comparable to the 
    // black and white squares you'd find on a chess board
    // I want to split up the 4000000 x 4000000 grid into two 4000000 long maps for black and white
    // This wouldn't really help that much by itself, but considering the fact that if we do it this way, we can jump across the
    // million wide diamonds around each sensor instead of iterating across each
    // Basically it moves it from n^2 to n (considering the number of sensors as constant time)

    // Break up the map into two diagonal pieces for the black and white squares
    let mut distress_coord = None;
    'full: for j in 0..(max_distance * 2) {
        // Each row in this iter is an odd number long: i * 2 + 1
        let (row_length, start_i) = if j < max_distance {
            (j + 1, 0)
        }
        else {
            ((max_distance * 2) - j, j - max_distance)
        };

        let mut i = start_i;
        'row: while i < (start_i + row_length) {
            // Determine the coordinate corresponding to this location
            let coord = Coord {
                x: i,
                y: j - i,
            };
            // println!("Checking coord (x: {}, y: {})...(i: {i}, j: {j})", coord.x, coord.y);
            // At each, loop through the sensors and check if this coord is in their range
            for (sensor_coord, _, range) in sensor_data.iter() {
                // Check if this square is in the sensor's range
                let dist_to_sensor = coord.manhattan_distance(sensor_coord);
                // If so, determine the width of this range (that is left for you to traverse) and jump i to the end of that diamond
                if dist_to_sensor <= *range {
                    // Find the diagonal "column" of the diamond that this coord is in and jump the rest of it to get just outside it
                    let dist_to_jump = 1 + *range as i32 - (*range as i32 - (sensor_coord.x - sensor_coord.y) + (coord.x - coord.y) + 1) / 2;
                    i += dist_to_jump;
                    continue 'row;
                }
            }
            // If none have been in a range for a given i, j, this is the distress coord!
            distress_coord = Some(Coord {
                x: i,
                y: j - i,
            });
            println!("Found the distress coord! It's {distress_coord:?}");
            break 'full;
        }
    }

    // Determine the coord
    let distress_coord = distress_coord.expect("Failed to find the coordinate of the distress beacon");

    // Calculate the tuning frequency given the coordinate of the distress beacon
    distress_coord.x as u64 * 4000000 + distress_coord.y as u64
}
