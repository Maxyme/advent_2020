// Find the total number of voltage difference to try all given voltage converters
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};


fn get_possible_combination(number: &i32, all_numbers: &Vec<i32>) -> i64 {
    // Return the number of possible combination for the given number. Expecting a 3 possible range
    // Note, this is very similar to Fibonacci
    fn get_possible_combination_cached(
        number: &i32,
        all_numbers: &Vec<i32>,
        cache: &mut HashMap<i32, i64>,
    ) -> i64 {
        match number {
            0 => 1,
            1 => 1,
            _ => {
                if !all_numbers.contains(number) {
                    // return early if the number is not included in the list
                    return 0;
                }
                match cache.get(number) {
                    // Try looking in the cache first
                    Some(x) => *x,
                    None => {
                        let mut sum: i64 = 0;
                        for num_diff in 1..=3 {
                            sum += get_possible_combination_cached(
                                &(number - num_diff),
                                all_numbers,
                                cache,
                            )
                        }
                        cache.insert(*number, sum);
                        sum
                    }
                }
            }
        }
    }
    let mut cache: HashMap<i32, i64> = HashMap::new();
    get_possible_combination_cached(number, all_numbers, &mut cache)
}

fn main() {
    let file = File::open("./src/input.txt").expect("Error reading file");
    let buf_reader = BufReader::new(file);

    let mut input_lines: Vec<i32> = buf_reader
        .lines()
        .into_iter()
        .map(|l| {
            l.ok()
                .and_then(|s| s.parse().ok())
                .expect("Expected valid int")
        })
        .collect();

    // make into an ordered list to iterate faster
    input_lines.sort_unstable();

    let mut one_jolt_diff_count = 0;
    let mut three_jolt_diff_count = 1; // the device output is always 3, so start at 1

    let mut previous = 0;
    for input in &input_lines {
        match input - previous {
            3 => three_jolt_diff_count += 1,
            1 => one_jolt_diff_count += 1,
            _ => panic!("Jump unexpected"),
        }
        previous = *input;
    }
    println!(
        "The number of 1-jolt differences multiplied by the number of 3-jolt differences {}",
        one_jolt_diff_count * three_jolt_diff_count
    );

    // part 2 - What is the total number of distinct ways you can arrange the adapters to connect the charging outlet to your device?
    // The jump can be 1,2 or 3 above
    let combination_count =
        get_possible_combination(&input_lines.last().expect("Expected Int"), &input_lines);
    println!(
        "The total number of possible combination is {}",
        combination_count
    );
}
