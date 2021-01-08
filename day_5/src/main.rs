/* Decode boarding pass numbers to find the highest seat id
seat: FBFBBFFRLR (first seven are row, last three are column)
seat ID: multiply the row by 8, then add the
F or L means lower half, while B or R means upper half
examples:
    BFFFBBFRRR: row 70, column 7, seat ID 567.
    FFFBBBFRRR: row 14, column 7, seat ID 119.
    BBFFBBFRLL: row 102, column 4, seat ID 820.

*/
use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;

fn get_seat_id(input: &str) -> i32 {
    // Convert to binary numbers
    let binary_row = input[0..7].replace("B", "1").replace("F", "0");
    let row = isize::from_str_radix(&*binary_row, 2).unwrap() as i32;
    let binary_colum = input[7..].replace("L", "0").replace("R", "1");
    let column = isize::from_str_radix(&*binary_colum, 2).unwrap() as i32;
    return row * 8 + column;
}

fn main() {
    let file_path = PathBuf::from("./src/input.txt");
    let f = fs::read_to_string(&file_path).expect("Error reading file");
    let input_lines: Vec<_> = f.lines().collect::<Vec<_>>(); // .lines() to split on lines

    let mut highest_seat_id = 0;
    let mut lowest_sead_id = f64::INFINITY; // Comparing to infinity to always get something lower
    let mut all_seat_ids = vec![];

    for input in input_lines {
        let seat_id = get_seat_id(input);
        if seat_id > highest_seat_id {
            highest_seat_id = seat_id;
        } else if seat_id < lowest_sead_id as i32 {
            lowest_sead_id = seat_id as f64
        }
        all_seat_ids.push(seat_id);
    }
    // Note: can also look for the maximum and minimum value of the vec:
    // ie: lowest_sead_id = all_seat_ids.iter().min().unwrap()
    println!("Highest seat id is {}", all_seat_ids.iter().min().unwrap());
    println!("Lowest seat id is {}", all_seat_ids.iter().max().unwrap());

    // part 2 - find missing seat in a full flight. Hint is that id +1 and -1 exist.
    let all_seat_ids: HashSet<i32> = all_seat_ids.into_iter().collect();
    let full_range = lowest_sead_id as i32..highest_seat_id;
    let all_possible_seat_ids: HashSet<i32> = full_range.into_iter().collect();

    let mut diff = all_possible_seat_ids.difference(&all_seat_ids);
    println!("missing seat id is: {}", diff.next().unwrap());
}
