/* Check if the password matches the password policy in the provided input */

use itertools::Itertools;
use std::fs;
use std::path::PathBuf;

fn password_valid(rule: &str, password: &str) -> bool {
    // Check the the provided password matches the rule
    let (full_range, char_to_check) = rule.splitn(2, char::is_whitespace).collect_tuple().unwrap(); //use itertools to get a tuple directly

    let (lower_range, upper_range) = full_range
        .split("-")
        .map(|i| i.parse().unwrap())
        .collect_tuple()
        .unwrap();

    return (lower_range..=upper_range).contains(&(password.matches(char_to_check).count() as i32));
}

fn password_valid_part2(rule: &str, password: &str) -> bool {
    // Check the the provided password matches the rule provided in part 2 of the problem
    let (full_range, char_to_check) = rule.splitn(2, char::is_whitespace).collect_tuple().unwrap();
    let char_to_check: char = char_to_check.chars().next().unwrap(); // convert the str to char

    let (first_index, second_index): (usize, usize) = full_range
        .split("-")
        .map(|i| i.parse::<usize>().unwrap())
        .collect_tuple()
        .unwrap();

    // extract the values at the specified index
    let password_char_first_index = {
        match password.chars().nth(first_index + 1) {
            Some(v) => v, // if value exists set
            None => return false, // return false if index outside of string
        }
    };

    let password_char_second_index = {
        match password.chars().nth(second_index + 1) {
            Some(v) => v, // if value exists set
            None => return false, // return false if index outside of string
        }
    };

    // add one because the problem uses 1-based indexing
    return password_char_first_index == char_to_check && password_char_second_index == char_to_check;
}

fn main() {
    let file_path = PathBuf::from("./src/input.txt");
    let file_path_str = &file_path.clone().into_os_string().into_string().unwrap();
    let error_message = format!("Error reading file at {}", file_path_str);
    let f = fs::read_to_string(&file_path).expect(&*error_message); // expect will return the error directly.
    println!(
        "Reading file at {:?}",
        fs::canonicalize(&file_path).unwrap()
    ); // canonicalize to get absolute path

    let input_values: Vec<_> = f.lines().collect::<Vec<_>>(); // .lines() to split on lines
    let mut valid_password_count = 0;
    let mut valid_password_count_part2 = 0;
    for input in input_values {
        let (rule, password) = input.split(":").collect_tuple().unwrap();
        if password_valid(rule, password) {
            valid_password_count += 1;
        }
        if password_valid_part2(rule, password) {
            valid_password_count_part2 += 1;
        }
    }
    println!(
        "There are {} valid passwords for part 1",
        valid_password_count
    );
    println!(
        "There are {} valid passwords for part 2",
        valid_password_count_part2
    );
}
