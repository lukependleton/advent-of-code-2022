fn main() {
    // Read in the received packets from the question trimming any surrounding whitespace
    let question_received_packets = include_str!("../inputs/question").trim();

    // * Part One
    // Get the sum of the indices of the packet pairs in the correct order in the input from the question
    let correct_packet_pair_index_sum = part_one(question_received_packets);
    println!("Part One:\n  The sum of packet pair indicies in the correct order is: {correct_packet_pair_index_sum}");

    // * Part Two
    // Order all received packts and find the product of the indices of the divider packets in the input from the question
    let divider_packets_index_product = part_two(question_received_packets);
    println!("Part Two:\n  The product of the divider packet indices among all ordered packets is: {divider_packets_index_product}");
}

// region: Helpers

#[derive(Clone, Debug, Eq)]
enum PacketData {
    List(Vec<PacketData>),
    Int(u32),
}

impl Ord for PacketData {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Self::Int(l0), Self::Int(r0)) => l0.cmp(r0),
            (Self::List(l0), Self::List(r0)) => {
                // Vector implements its PartialOrd lexographically, which reading the description is what we want for this problem
                // Found it here: https://doc.rust-lang.org/std/vec/struct.Vec.html#impl-PartialOrd%3CVec%3CT%2C%20A%3E%3E-for-Vec%3CT%2C%20A%3E
                l0.cmp(r0)
            }
            // If exactly one of the sides is an int, we want to construct a list with only that int in it for comparison
            (Self::List(l0), Self::Int(r0)) => {
                Self::List(l0.to_vec()).cmp(&Self::List(vec![Self::Int(*r0)]))
            }
            (Self::Int(l0), Self::List(r0)) => {
                Self::List(vec![Self::Int(*l0)]).cmp(&Self::List(r0.to_vec()))
            }
        }
    }
}

impl PartialOrd for PacketData {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for PacketData {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::List(l0), Self::List(r0)) => l0 == r0,
            (Self::Int(l0), Self::Int(r0)) => l0 == r0,
            // If exactly one of the sides is an int, we want to construct a list with only that int in it for checking equality
            (Self::List(l0), Self::Int(r0)) => {
                Self::List(l0.to_vec()) == Self::List(vec![Self::Int(*r0)])
            }
            (Self::Int(l0), Self::List(r0)) => {
                Self::List(vec![Self::Int(*l0)]) == Self::List(r0.to_vec())
            }
        }
    }
}

/// Parse the input list of received packet info into a vector of packet pairs
fn parse_received_packets(received_packets: &str) -> Vec<(Vec<PacketData>, Vec<PacketData>)> {
    received_packets
        .split("\n\n")
        .map(|packet_pair| {
            let (left, right) = packet_pair
                .split_once("\n")
                .expect("Invalid packet pair structure - bad input");

            // Parse the left and right, making sure that they are both Lists at the top level
            (parse_packet_list(left), parse_packet_list(right))
        })
        .collect()
}

/// Parse a str representing a packet list into a vector of its correspoinding PacketData
fn parse_packet_list(packet: &str) -> Vec<PacketData> {
    // Affirm that the packet str start with '[' and ends with ']' and remove them
    if !packet.starts_with('[') || !packet.ends_with(']') {
        panic!(
            "The packet str input needs to be a list -> needs to start with '[' and end with ']'"
        );
    }

    // Remove the first and last chars now that we know they are the square brackets
    let mut packer_chars = packet.chars();
    packer_chars.next();
    packer_chars.next_back();
    let packet = packer_chars.as_str();

    // Split the string on "same level" commas - commas related to the current list
    // For example, this would split: "1,2,[3,4],5" into: ["1", "2", "[3,4]", "5"]
    let mut level = 0;
    let list_split = packet.split(|c| match c {
        '[' => {
            // Increment the level of lists we are currently in
            level += 1;
            false
        }
        ']' => {
            // Decrement the level of lists we are currently in
            level -= 1;
            false
        }
        ',' => level == 0,
        _ => false,
    });

    // Recursively parse each PacketData element of the list and return the collected result
    list_split
        .filter_map(|packet_data| {
            (!packet_data.is_empty()).then(|| {
                if packet_data.starts_with('[') {
                    // Create new List packet data and recursively call parse_packet_list to determine its contents
                    PacketData::List(parse_packet_list(packet_data))
                } else if let Some(integer) = packet_data.to_string().parse::<u32>().ok() {
                    PacketData::Int(integer)
                } else {
                    panic!("invalid element <{packet_data}>");
                }
            })
        })
        .collect::<Vec<_>>()
}

// endregion

// region: Part One

#[test]
fn part_one_example_test() {
    // Read in the example received packets trimming any surrounding whitespace
    let example_received_packets = include_str!("../inputs/example").trim();

    // Get the sum of the indices of the packet pairs in the correct order in the example input from the question
    let correct_packet_pair_index_sum = part_one(example_received_packets);

    // Check if the example yields the same result as the question describes
    assert_eq!(correct_packet_pair_index_sum, 13);
}

/// Find the sum of the indices of the packet pairs in the correct order
fn part_one(received_packets: &str) -> u32 {
    // Parse the input list of received packet info into a vector of packet pairs
    let parsed_received_packets = parse_received_packets(received_packets);

    // Check to see whether each packet pair is in the right order and sum the indices (+ 1) of the ones that are
    parsed_received_packets
        .iter()
        .enumerate()
        .map(|(i, (left_packet, right_packet))| {
            // Compare the two packets returning the index if they are in the correct order or zero if they are not
            if left_packet < right_packet {
                // The "indices" the question is looking for are 1-indexed
                1 + i as u32
            } else {
                0
            }
        })
        .sum()
}

// endregion

// region: Part Two

#[test]
fn part_two_example_test() {
    // Read in the example received packets trimming any surrounding whitespace
    let example_received_packets = include_str!("../inputs/example").trim();

    // Order all received packts and find the product of the indices of the divider packets in the example input from the question
    let divider_packets_index_product = part_two(example_received_packets);

    // Check if the example yields the same result as the question describes
    assert_eq!(divider_packets_index_product, 140);
}

/// Find the product of the indices of the divider packets among the correctly ordered list of all received packets
fn part_two(received_packets: &str) -> u32 {
    // Parse the input list of received packet info into a vector of packet pairs
    let parsed_received_packets = parse_received_packets(received_packets);

    // Combine the parsed packets pairs together into one big vector
    let mut received_packets = parsed_received_packets
        .into_iter()
        .flat_map(|(left_packet, right_packet)| [left_packet, right_packet].into_iter())
        .collect::<Vec<_>>();

    // Add the additional divider packets [[2]] and [[6]]
    let additional_divider_packets = ["[[2]]", "[[6]]"]
        .into_iter()
        .map(|packet_str| parse_packet_list(packet_str))
        .collect::<Vec<_>>();

    for packet in additional_divider_packets.iter() {
        received_packets.push(packet.clone());
    }

    // Sort the list of received packets plus divider packets
    received_packets.sort();

    // Return the product of the indices of the divider packets
    additional_divider_packets
        .iter()
        .map(|divider_packet| {
            // The "indices" the question is looking for are 1-indexed
            1 + received_packets
                .iter()
                .position(|packet| *packet == *divider_packet)
                .expect("Failed to find divider packet") as u32
        })
        .product()
}

// endregion
