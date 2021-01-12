// Find the number of permutations required to have a stable seating arrangement
use std::io::{BufRead, BufReader};

use std::collections::hash_map::DefaultHasher;
use std::fs::File;
use std::hash::{Hash, Hasher};

fn get_num_occupied(index: usize, line_length: usize, layout: &Vec<char>, check_far: bool) -> i32 {
    let directions: [(i32, i32); 8] = [
        (1, 0),
        (-1, 0),
        (-1, -1),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 1),
    ];
    let mut num_occupied = 0;
    let mut new_index = 0;

    for (mut x, mut y) in directions.iter() {
        let mut previous_index = index as i32;
        loop {
            let new_index = new_index + index as i32 + x + (y * line_length as i32);

            // Check if the index would be out of range, or that a column has been crossed
            if new_index >= layout.len() as i32
                || new_index < 0
                || previous_index % line_length as i32 == 0
                    && (new_index + 1) % line_length as i32 == 0
                || (previous_index + 1) % line_length as i32 == 0
                    && new_index % line_length as i32 == 0
            {
                break;
            }

            let value = layout[new_index as usize];
            if value == '#' {
                num_occupied += 1;
                break;
            } else if value == 'L' {
                break;
            }
            if !check_far {
                break;
            }
            x += x;
            y += y;
            previous_index = new_index;
        }
    }
    num_occupied
}

fn print_layout(layout: &Vec<char>, line_length: usize) {
    // Used to debug the layout changes
    for (index, item) in layout.iter().enumerate() {
        if index % line_length == 0 {
            println!();
        }
        print!("{}", item);
    }
    println!();
}

fn hash_vec(layout: &Vec<char>) -> u64 {
    // Hash a vector to check the contents changes
    let mut hasher = DefaultHasher::new();
    layout.hash(&mut hasher);
    hasher.finish()
}

fn get_new_layout(
    layout: &Vec<char>,
    line_length: usize,
    check_far: bool,
    limit: i32,
) -> Vec<char> {
    /*
    Return a new layout following the seating rules

    If a seat is empty (L) and there are no occupied seats adjacent to it, the seat becomes occupied.
    If a seat is occupied (#) and four or more seats adjacent to it are also occupied, the seat becomes empty.
    Otherwise, the seat's state does not change.
    Note: adjacent to a given seat means one of the eight positions immediately up, down, left, right, or diagonal from the seat
    */
    //print_layout(layout, line_length);
    let mut return_layout: Vec<char> = Vec::with_capacity(layout.len());
    for (index, item) in layout.iter().enumerate() {
        if item == &'.' {
            return_layout.push(*item);
            continue;
        }

        let number_occupied = get_num_occupied(index, line_length, layout, check_far);
        if number_occupied == 0 {
            return_layout.push('#');
        } else if number_occupied >= limit {
            return_layout.push('L');
        } else {
            return_layout.push(*item);
        }
    }
    return_layout
}

fn get_sum_occupied(lines: &Vec<String>, check_far: bool, limit: i32) -> i32 {
    // Flatten into a flat char vec
    let mut layout: Vec<char> = lines.iter().map(|s| s.chars()).flatten().collect();
    loop {
        let old_hash = hash_vec(&layout);
        layout = get_new_layout(&layout, lines[0].len(), check_far, limit);

        if hash_vec(&layout) == old_hash {
            break;
        }
    }

    // Count the number of occupied seats after no more permutations are needed
    let sum_occupied: usize = layout.iter().filter(|&x| *x == '#').count();
    sum_occupied as i32
}

fn main() {
    //let file = File::open("./src/input.txt").expect("Unable to open file");
    let file = File::open("./src/input.txt").expect("Unable to open file");
    let reader = BufReader::new(file);
    let lines: Vec<_> = reader
        .lines()
        .map(|l| l.expect("Unable to read line"))
        .collect::<Vec<_>>(); // split on empty lines

    // Part 1
    // Only check the absolute nearby seat
    //let sum_occupied_nearby = get_sum_occupied(&lines, false, 4);
    //println!("{} occupied seats for part 1", sum_occupied_nearby);

    // Part 2
    // Visible seats are in the full vector of top, down, left right, diagonal
    // instead of directly next to the selected seat
    let sum_occupied_line = get_sum_occupied(&lines, true, 5);
    println!("{} occupied seats for part 2", sum_occupied_line);
}
