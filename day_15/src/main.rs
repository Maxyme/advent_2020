/*
Find the nth spoken number in a infinite number's game with the following rules:
Players take turns saying numbers from list of starting numbers then, each turn consists of considering the most recently spoken number:
If that was the first time the number has been spoken, the current player says 0.
Otherwise, the number had been spoken before; the current player announces how many turns apart the number is from when it was previously spoken.
 */

use std::collections::HashMap;
use std::collections::VecDeque;
use std::fs;
use std::path::PathBuf;

fn main() {
    let file_path = PathBuf::from("./src/input.txt");
    let f = fs::read_to_string(&file_path).expect("Error reading file");
    let lines: Vec<&str> = f.lines().collect::<Vec<_>>();

    // add numbers to queue
    let mut first_numbers: VecDeque<u32> = lines[0]
        .split(",")
        .map(|i| i.parse().expect("Expected int"))
        .collect();

    // Part 1 is 2020, Part 2 is 30000000
    let count = 30000000;

    let mut numbers_indexes: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut last_number_spoken = 0;
    let mut counter = 0;

    loop {
        if counter == count {
            break;
        }
        match first_numbers.pop_front() {
            Some(x) => {
                // Update the index values for the first given numbers
                last_number_spoken = x;
                let indexes = numbers_indexes.entry(x).or_insert(vec![]);
                indexes.push(counter);
            }
            None => {
                // After first_numbers are processed
                match numbers_indexes.get(&last_number_spoken) {
                    Some(x) => {
                        let mut entry = 0;
                        if x.len() > 1 {
                            // Subtract before_last_spoken_index from last_spoken_index
                            entry = x[x.len() - 1] - x[x.len() - 2];
                        }

                        last_number_spoken = entry;
                        let indexes = numbers_indexes.entry(entry).or_insert(vec![]);
                        indexes.push(counter);
                    }
                    None => {
                        panic!("Should never be false");
                    }
                }
            }
        }
        counter += 1;
    }

    println!(
        "The last number spoken for count {} is {}",
        count, last_number_spoken
    );
}
