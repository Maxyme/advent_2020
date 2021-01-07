/* find the two entries in the list of inputs that sum to 2020 and then multiply those two numbers together */

use std::fs;
use std::path::PathBuf;

fn main() {
    let file_path = PathBuf::from("./src/input.txt");
    let file_path_str = &file_path.clone().into_os_string().into_string().unwrap();
    let error_message = format!("Error reading file at {}", file_path_str);
    let f = fs::read_to_string(&file_path).expect(&*error_message); // expect will return the error directly.
    println!(
        "Reading file at {:?}",
        fs::canonicalize(&file_path).unwrap()
    ); // canonicalize to get absolute path

    let input_values: Vec<_> = f.lines().map(|i| i.parse().unwrap()).collect::<Vec<u32>>(); // .lines() to split on lines and map values to u32

    // product of 2 entries that sum up to 2020
    // label outer loop to be able to break nested loops
    'outer_1: for (index, first_value) in input_values.iter().enumerate() {
        for second_value in &input_values[index..] {
            if first_value + second_value == 2020 {
                println!(
                    "Found total of {} from value {} and value {}",
                    first_value * second_value,
                    first_value,
                    second_value
                );
                break 'outer_1;
            }
        }
    }
    // product of 3 entries that sum up to 2020
    'outer_2: for (index, first_value) in input_values.iter().enumerate() {
        for second_value in &input_values[index..] {
            for third_value in &input_values[index + 1..] {
                if first_value + second_value + third_value == 2020 {
                    println!(
                        "Found total of {} from value_1 {}, value_2 {} and value_3 {}",
                        first_value * second_value * third_value,
                        first_value,
                        second_value,
                        third_value
                    );
                    break 'outer_2;
                }
            }
        }
    }
}

// One liner example from https://github.com/timvisee/advent-of-code-2020/blob/master/day01a/src/main.rs as reference
// use itertools::Itertools;
// pub fn main_2() {
//     println!(
//         "{}",
//         include_str!("../input.txt")
//             .lines()
//             .map(|i| i.parse::<usize>().unwrap())
//             .combinations(2)
//             .filter(|i| i.iter().sum::<usize>() == 2020)
//             .next()
//             .map(|i| i.iter().product::<usize>())
//             .unwrap()
//     );
// }
