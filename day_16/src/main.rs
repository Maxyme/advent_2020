/*
Determine the ticket scanning error rate by considering the validity of the nearby tickets scanned
with the following rule:
class: 1-3 or 5-7, means that ticket 1,2,3 or 5,6,7 would be valid, but not 4
 */

use itertools::Itertools;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::iter::FromIterator;
use std::path::PathBuf;

fn get_column_ticket_values_for_index(index: usize, tickets: &Vec<Vec<i32>>) -> Vec<i32> {
    let mut values = Vec::new();
    for ticket in tickets {
        values.push(ticket[index]);
    }
    values
}

fn get_invalid_values(values: &Vec<i32>, ranges: &Vec<(i32, i32)>) -> Vec<i32> {
    // Calculate all the possible values for the given ranges
    let mut valid_numbers: Vec<i32> = Vec::new();
    for (lower_range, upper_range) in ranges {
        for value in *lower_range..=*upper_range {
            valid_numbers.push(value);
        }
    }

    // Return a list of all the missing tickets from the valid list
    let mut invalid_tickets: Vec<i32> = Vec::new();
    for value in values {
        if !valid_numbers.contains(&value) {
            invalid_tickets.push(*value);
        }
    }

    invalid_tickets
}

fn filter_line_columns(line_columns: &HashMap<usize, Vec<usize>>) -> HashMap<usize, usize> {
    // Filter down the dict so that only one column value is available per validity line
    let mut seen_values: HashSet<usize> = HashSet::new();
    let mut line_column = HashMap::new();
    for value_len in 1..20 {
        for (key, value) in line_columns {
            if value.len() == value_len {
                // let the single value become the value that is missing from the seen list
                let set = HashSet::from_iter(value.iter().cloned());
                {
                    let single_value = set.difference(&seen_values).next().expect("Expected int");
                    line_column.insert(*key, *single_value);
                }
                seen_values = set;
                break;
            }
        }
    }

    line_column
}

fn main() {
    let file_path = PathBuf::from("./src/input.txt");
    let f = fs::read_to_string(&file_path).expect("Error reading file");

    // Separate into 3 fields divided by an empty line
    let input_entries: Vec<&str> = Regex::new(r"\n\n").unwrap().split(&f).collect::<Vec<_>>(); // split on empty lines
    let fields = input_entries[0].lines().collect::<Vec<_>>();
    let re = Regex::new(r"[0-9]*-[0-9]*").expect("Invalid regex");

    // Parse the valid ranges
    let mut valid_ranges: Vec<(i32, i32)> = Vec::new();
    for field in fields {
        let range_texts: Vec<&str> = re.find_iter(field).map(|mat| mat.as_str()).collect();
        for range_text in range_texts {
            // Collect a tuple of lower and upper ranges
            let (lower_range, upper_range): (i32, i32) = range_text
                .split("-")
                .map(|i| i.parse().unwrap())
                .collect_tuple()
                .expect("Invalid tuple");
            valid_ranges.push((lower_range, upper_range))
        }
    }

    // Parse all nearby tickets into a list and get all the tickets that are not in the valid list
    let mut valid_tickets: Vec<Vec<i32>> = Vec::new();
    let mut invalid_values_sum: i32 = 0;
    let nearby_tickets_parts = &input_entries[2].lines().collect_vec();
    for ticket in &nearby_tickets_parts[1..] {
        let ticket_values: Vec<i32> = ticket
            .split(",")
            .map(|x| x.parse().expect("Expected i32"))
            .collect_vec();
        let invalid_ticket_values = get_invalid_values(&ticket_values, &valid_ranges);

        // If no invalid values, then add to the valid tickets
        if invalid_ticket_values.len() == 0 {
            valid_tickets.push(ticket_values);
        } else {
            invalid_values_sum += invalid_ticket_values.iter().sum::<i32>()
        }
    }

    println!(
        "The sum of invalid tickets values is {}",
        invalid_values_sum
    );

    // Part 2, determine what the fields are used for your ticket with the word 'departure' and multiply them together

    // Get all the values in "your" ticket from the line column dict
    let your_ticket_lines = &input_entries[1].lines().collect_vec();
    let your_ticket_values: Vec<i32> = your_ticket_lines[1]
        .split(",")
        .collect_vec()
        .iter()
        .map(|x| x.parse().expect("Expected Int"))
        .collect_vec();

    // Iterate through the first ranges to get all possible valid column values
    let mut line_columns = HashMap::new();
    for line_index in 0..20 {
        //6 {
        // Get the valid values for the line
        let ranges = valid_ranges[(2 * line_index)..(2 * line_index + 2)].to_vec();

        // Iterate the columns to see if they are matching the valid tickets
        for index in 0..your_ticket_values.len() {
            let tickets_for_column: Vec<i32> =
                get_column_ticket_values_for_index(index, &valid_tickets);

            let invalid_ticket_values = get_invalid_values(&tickets_for_column, &ranges);
            if invalid_ticket_values.len() == 0 {
                let columns = line_columns.entry(line_index).or_insert(vec![]);
                columns.push(index);
            }
        }
    }

    // Filter the line index, so that each line has 1 column
    let line_column = filter_line_columns(&line_columns);

    // Multipling the values for the lines starting with description
    let mut multiplication_value: i64 = 1;
    for line_index in 0..6 {
        let column_index = line_column[&line_index];
        multiplication_value *= your_ticket_values[column_index] as i64;
    }

    println!("The multiplication total is {}", &multiplication_value);
}
