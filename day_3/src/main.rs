/* Check the number of # encountered in the input file when crossing from top left following a vector
Note, the .# values repeat horizontally if out of the index.
*/

use std::fs;

const TREE: char = '#';

fn get_number_trees_hit(input_lines: &Vec<&str>, direction: &[usize]) -> usize {
    let mut number_of_tree = 0;
    let mut right_counter = 0;
    for (index, line) in input_lines.iter().enumerate() {
        if index % direction[1] != 0 {
            continue; // skip every nth down line
        }
        if line
            .chars()
            .nth(right_counter % line.len())
            .expect("No result found")
            == TREE
        {
            // hit a tree!
            number_of_tree += 1;
        }
        right_counter += direction[0];
    }
    return number_of_tree;
}

fn main() {
    let f = fs::read_to_string("./src/input.txt").expect("Error reading file");
    let input_lines: Vec<&str> = f.lines().collect();

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
