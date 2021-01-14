/*
Part 1:
Mask will modify the value
Return the sum of all values left in memory after it completes
a 0 or 1 overwrites the corresponding bit in the value, while an X leaves the bit in the value unchanged.

Part 2:
Mask modifies the address instead of the value
Return the sum of all values left in memory after it completes:
If the bitmask bit is 0, the corresponding memory address bit is unchanged.
If the bitmask bit is 1, the corresponding memory address bit is overwritten with 1.
If the bitmask bit is X, the corresponding memory address bit is floating.
*/

use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::fs;
use std::path::PathBuf;

fn apply_mask_value(value: isize, mask: &str) -> isize {
    // Apply a binary mask to a value
    let bin_idx = format!("{:b}", value);

    // Use a deque to push on the left side
    let mut deque: VecDeque<char> = VecDeque::with_capacity(mask.len());
    for (index, value) in mask.chars().rev().enumerate() {
        match value {
            'X' => {
                if index > bin_idx.len() - 1 {
                    deque.push_front('0')
                } else {
                    // get the original value if it exists else push 0
                    let original_value = match bin_idx.chars().nth(bin_idx.len() - index - 1) {
                        Some(v) => v,
                        None => '0',
                    };
                    deque.push_front(original_value);
                }
            }
            '0' => deque.push_front('0'),
            '1' => deque.push_front('1'),
            _ => panic!("Error"),
        }
    }
    let bin: String = deque.iter().collect();
    isize::from_str_radix(&*bin, 2).expect("Expected binary")
}

fn apply_address_mask(value: isize, mask: &str) -> String {
    // apply the mask for an address for part 2
    let bin_idx = format!("{:b}", value);

    // Use a deque to push on the left side
    let mut deque: VecDeque<char> = VecDeque::with_capacity(mask.len());
    for (index, value) in mask.chars().rev().enumerate() {
        match value {
            'X' => deque.push_front('X'),
            '0' => {
                if index > bin_idx.len() - 1 {
                    deque.push_front('0')
                } else {
                    // get the original value if it exists else push 0
                    let original_value = match bin_idx.chars().nth(bin_idx.len() - index - 1) {
                        Some(v) => v,
                        None => '0',
                    };
                    deque.push_front(original_value);
                }
            }
            '1' => deque.push_front('1'),
            _ => panic!("Error"),
        }
    }
    // Return deque collected into a String
    deque.iter().collect()
}

fn get_addresses(address: isize, mask: &str) -> Vec<String> {
    // Return all possible addresses for a specific address and a given mask
    let masked_address: String = apply_address_mask(address, mask);

    // Count the number of X's in the masked address to create a return vec
    let count_x = mask.chars().filter(|x| x == &'X').count();
    let count_address = 2_usize.pow(count_x as u32);

    // Create a vector containing copies of the masked addresses to edit later
    let mut addresses = vec![masked_address; count_address];

    // Get all possible combinations of binary values for the x count
    let mut permutations = Vec::with_capacity(count_address);
    for index in 0..count_address {
        // front pad with the right number of zeros
        let str1 = format!("{:0width$b}", index, width = count_x);
        permutations.push(str1);
    }

    // Update each X in string in the return vector with the permutations of 0's and 1's
    for (first_index, permutation) in permutations.iter().enumerate() {
        for value in permutation.chars() {
            addresses[first_index] = addresses[first_index].replacen("X", &*value.to_string(), 1);
        }
    }

    addresses
}

fn main() {
    let file_path = PathBuf::from("./src/input.txt");
    let f = fs::read_to_string(&file_path).expect("Error reading file");
    let lines: Vec<(&str, &str)> = f
        .lines()
        .map(|x| {
            x.splitn(2, " = ")
                .collect_tuple()
                .expect("Expected 2 values")
        })
        .collect::<Vec<_>>();

    // Part 1 - mask modifies the value
    let re = Regex::new(r"mem\[([0-9]*?)\]").expect("Regex not found");
    let mut addresses_values: HashMap<&str, isize> = HashMap::new();
    let mut mask: &str = "";
    for (instruction, value) in &lines {
        if instruction == &"mask" {
            mask = value;
        } else {
            let value_as_num = value.parse::<isize>().expect("Expect int");
            let new_value = apply_mask_value(value_as_num, mask);
            let mem_address = re
                .captures(instruction)
                .expect("Regex error")
                .get(1)
                .expect("No match")
                .as_str();
            let stat = addresses_values.entry(mem_address).or_insert(new_value);
            *stat = new_value;
        }
    }

    println!(
        "The sum of all memory values for part 1 is {}",
        addresses_values.values().sum::<isize>()
    );

    // Part 2 - the mask modifies the memory addresses instead of the values
    let mut addresses_values: HashMap<String, usize> = HashMap::new();
    let mut mask: &str = "";
    for (instruction, value) in &lines {
        if instruction == &"mask" {
            mask = value;
        } else {
            let mem_address = re
                .captures(instruction)
                .expect("Regex error")
                .get(1)
                .expect("No match")
                .as_str();
            let mem_address = mem_address.parse::<isize>().expect("Expect int");
            let mem_addresses = get_addresses(mem_address, mask);
            let value_as_num = value.parse::<usize>().expect("Expect int");
            for mem_address in mem_addresses.iter() {
                let stat = addresses_values
                    .entry(mem_address.clone())
                    .or_insert(value_as_num);
                *stat = value_as_num;
            }
        }
    }

    println!(
        "The sum of all memory values for part 2 is {}",
        addresses_values.values().sum::<usize>()
    );
}
