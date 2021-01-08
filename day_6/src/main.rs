use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::PathBuf;

fn main() {
    let file_path = PathBuf::from("./src/input.txt");
    let f = fs::read_to_string(&file_path).expect("Error reading file");
    let input_entries: Vec<_> = Regex::new(r"\n\n").unwrap().split(&f).collect::<Vec<_>>(); // split on empty lines

    // part 1 - create a hashset from the input entries to get any questions that received a yes
    let mut sum_count = 0;
    for entry in &input_entries {
        let group_count: HashSet<_> = entry.replace("\n", "").chars().into_iter().collect();
        sum_count += group_count.len();
    }

    println!("The sum of all counts for part 1 is {}", sum_count);

    // part 2 - create a dictionary with count from the input entries to get all questions that received a yes
    let mut sum_count = 0;
    for entry in input_entries {
        let lines = entry.lines().collect::<Vec<_>>();

        // make a hashmap of each key and their count and add 1 if the count equals the line length
        let mut char_counts = HashMap::new();
        for char in entry.replace("\n", "").chars().into_iter() {
            let counter = char_counts.entry(char).or_insert(0);
            *counter += 1;
            if counter == &lines.len() {
                sum_count += 1;
            }
        }
    }
    println!("The sum of all counts for part 2 is {}", sum_count);
}
