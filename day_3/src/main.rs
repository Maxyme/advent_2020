/* Check the number of # encountered in the input file when crossing from top left following a vector
Note, the .# values repeat horizontally if out of the index.
*/

use std::fs;
use std::path::PathBuf;

fn get_number_trees_hit(input_lines: &Vec<&str>, direction: &[i32]) -> i32 {
    let tree: char = "#".chars().next().unwrap();
    let mut number_of_tree: i32 = 0;
    let mut right_counter = 0;
    for (index, line) in input_lines.iter().enumerate() {
        if index % direction[1] as usize != 0 {
            continue; // skip every nth down line
        }
        let hit_tree = {
            match line.chars().nth(right_counter % line.len()) {
                Some(v) => v == tree,
                None => false,
            }
        };
        if hit_tree {
            // hit a tree!
            number_of_tree += 1;
        }
        right_counter += direction[0] as usize;
    }
    return number_of_tree;
}

fn main() {
    let file_path = PathBuf::from("./src/input.txt");
    let f = fs::read_to_string(&file_path).expect("Error reading file");
    let input_lines: Vec<_> = f.lines().collect::<Vec<_>>(); // .lines() to split on lines

    let direction = [3, 1]; // right 3 and down 1
    let number_trees_hit = get_number_trees_hit(&input_lines, &direction);

    println!("Number of trees hit = {}", number_trees_hit);
    // Part 2 - the total number of trees hit with the listed slopes
    /*
    Right 1, down 1.
    Right 3, down 1. (This is the slope you already checked.)
    Right 5, down 1.
    Right 7, down 1.
    Right 1, down 2.
    */
    let slopes = [[1, 1], [3, 1], [5, 1], [7, 1], [1, 2]];
    let mut total_multiplied = 1; // start at 1 since it will be multiplied
    for slope in slopes.iter() {
        let total = get_number_trees_hit(&input_lines, slope);
        total_multiplied *= total;
    }
    println!("Multiple num of trees hit = {}", total_multiplied);
}
