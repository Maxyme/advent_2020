/*
Part 1:
What is the ID of the earliest bus you can take to the airport multiplied by the number of minutes you'll need to wait for that bus?
ie. find the next number after the given timestamp that has the lowest denominator

Part 2:
What is the earliest timestamp such that all of the listed bus IDs depart at offsets matching their positions in the list?
ie. Find the timestamp that matches the list where the index is the increment of time where that bus departs
 */

use std::fs;
use std::path::PathBuf;

fn main() {
    let file_path = PathBuf::from("./src/input.txt");
    let f = fs::read_to_string(&file_path).expect("Error reading file");
    let lines: Vec<&str> = f.lines().collect::<Vec<_>>();

    let possible_buses: Vec<i32> = lines[1]
        .split(",")
        .filter(|x| x != &"x")
        .map(|i| i.parse().expect("Expected int"))
        .collect();

    // Part 1
    let timestamp: i32 = lines[0].parse().expect("Expected int");
    let mut earliest_bus = i32::MAX; // identical to f64::INFINITY as i32
    let mut next_possible_bus_timestamp = i32::MAX;
    for bus in possible_buses {
        let next_bus_timestamp = timestamp + bus - (timestamp % bus);
        if next_bus_timestamp < next_possible_bus_timestamp {
            earliest_bus = bus;
            next_possible_bus_timestamp = next_bus_timestamp;
        }
    }
    println!(
        "Earliest possible bus is {} at {}",
        earliest_bus, next_possible_bus_timestamp
    );
    println!(
        "Bus id multiplied by wait time is {}",
        earliest_bus * (next_possible_bus_timestamp - timestamp)
    );

    // Part 2
    let possible_buses: Vec<&str> = lines[1].split(",").collect();
    let mut start_stamp = 1;
    let mut increment = 1;

    for (index, bus) in possible_buses.iter().enumerate() {
        if bus == &"x" {
            // skip x
            continue;
        }
        let bus_id = bus.parse::<usize>().expect("Expected u32");
        loop {
            if (timestamp + index) % bus_id == 0 {
                // Found a multiplier of the bus id, update the increment and break
                increment *= bus_id;
                break;
            }
            start_stamp += increment;
        }
    }
    println!("Start value is {}", timestamp);
}
