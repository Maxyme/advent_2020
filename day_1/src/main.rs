/* find the two entries in the list of inputs that sum to 2020 and then multiply those two numbers together */

use std::fs;

fn main() {
    let input = fs::read_to_string("./src/input.txt").expect("Error reading file");
    let input_values: Vec<usize> = input.lines().map(|i| i.parse().unwrap()).collect();
    // product of 2 entries that sum up to 2020

    // label outer loop to be able to break nested loops
    let product_2_values: usize = {
        let mut multiple = 0;
        'outer_1: for (index, first_value) in input_values.iter().enumerate() {
            for second_value in &input_values[index..] {
                if first_value + second_value == 2020 {
                    multiple = first_value * second_value;
                    break 'outer_1;
                }
            }
        }
        multiple
    };
    println!("Product of 2 values that add up to 2020 is {}", product_2_values);

    // product of 3 entries that sum up to 2020
    let product_3_values: usize = {
        let mut multiple = 0;
        'outer_2: for (index, first_value) in input_values.iter().enumerate() {
            for second_value in &input_values[index..] {
                for third_value in &input_values[index + 1..] {
                    if first_value + second_value + third_value == 2020 {
                        multiple = first_value * second_value * third_value;
                        break 'outer_2;
                    }
                }
            }
        }
        multiple
    };

    println!("Product of 3 values that add up to 2020 is {}", product_3_values);
}
