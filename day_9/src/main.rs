// Find the first number in the list (after the preamble of 25 numbers) which is not the sum of two of the 25 numbers before it
use std::fs::File;
use std::io::{BufRead, BufReader};
fn can_be_sum_of_2_previous_from_list(number: &i64, list_of_previous: &[i64]) -> bool {
    let can_sum = false;
    for (index, first_number) in list_of_previous.iter().enumerate() {
        for second_number in list_of_previous[index..].iter() {
            if first_number + second_number == *number {
                return true;
            }
        }
    }
    can_sum
}

fn main() {
    let file = File::open("./src/input.txt").expect("Error reading file");
    let buf_reader = BufReader::new(file);

    let input_lines: Vec<i64> = buf_reader
        .lines()
        .into_iter()
        .map(|l| {
            l.ok()
                .and_then(|s| s.parse().ok())
                .expect("Expected valid int")
        })
        .collect();

    let preamble = 25;

    let mut problematic_num = 0;
    for (index, input) in input_lines[preamble..].iter().enumerate() {
        let real_index = index + preamble;
        let previous_values: &[i64] = &input_lines[real_index - preamble..real_index];
        let can_sum = can_be_sum_of_2_previous_from_list(input, previous_values);
        if !can_sum {
            println! {"Cannot find a sum in the previous list for {} with previous values {:?}", input, previous_values}
            problematic_num = *input;
            break;
        }
    }

    // part 2 - find the sum of the smallest and largest number in a contiguous range of at least 2 numbers
    // that add up to the problematic number found in step 1
    let mut partial_sum;
    let mut index_lookup = 0;
    let mut first_index = 0;
    'outer_1: for (index, input) in input_lines.iter().enumerate() {
        partial_sum = *input;
        index_lookup = index;
        loop {
            index_lookup += 1;
            partial_sum += input_lines[index_lookup];
            if partial_sum == problematic_num {
                // found the sum, return the first index
                first_index = index;
                break 'outer_1;
            } else if partial_sum > problematic_num {
                // partial sum got bigger than the desired number
                break;
            }
        }
    }

    // find smallest and largest values of sum
    let slice = &input_lines[first_index..=index_lookup];
    let smallest = slice.iter().min().expect("Expected Value!");
    let largest = slice.iter().max().expect("Expected Value!");
    println! {"Found a contiguous sum that adds up to {} with values {:?}", problematic_num, slice.iter()}
    println! {"Sum of smallest and largest number in range: {}", smallest + largest}
}
