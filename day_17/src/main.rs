/*
Starting with your given initial configuration, simulate six cycles. How many cubes are left in the active state after the sixth cycle?
Each cube only ever considers its neighbors: any of the 26 other cubes where any of their coordinates differ by at most 1.
Each cube either has an active (#) or inactive (.) state.
During a cycle, all cubes simultaneously change their state according to the following rules:
If a cube is active and exactly 2 or 3 of its neighbors are also active, the cube remains active. Otherwise, the cube becomes inactive.
If a cube is inactive but exactly 3 of its neighbors are active, the cube becomes active. Otherwise, the cube remains inactive.

Todo: use ndim logic instead with ArrayD for array 3

 */
use ndarray::{s, Array3, ArrayD, Dimension, IxDyn};
use std::fs;
use std::path::PathBuf;

fn get_padded_array(a: &Array3<i32>) -> Array3<i32> {
    // Return a padded array of the input array
    let new_dim = (a.dim().0 + 2, a.dim().1 + 2, a.dim().2 + 2);
    let mut b = Array3::<i32>::zeros(new_dim);
    b.slice_mut(s![1..-1, 1..-1, 1..-1])
        .assign(&a.slice(s![.., .., ..]));
    b
}

fn get_valid_neighbors(indexes: (usize, usize, usize), matrix: &Array3<i32>) -> i32 {
    // Return the number of active neighbours.
    let (x, y, z) = indexes;
    let mut count_valid = 0;
    for z_val in z as isize - 1..=z as isize + 1 {
        for y_val in y as isize - 1..=y as isize + 1 {
            for x_val in x as isize - 1..=x as isize + 1 {
                if z_val < 0 || y_val < 0 || x_val < 0 {
                    // out of bounds
                    continue;
                }
                if (x_val, y_val, z_val) == (x as isize, y as isize, z as isize) {
                    // don't count index value itself!
                    continue;
                }
                match matrix.get([x_val as usize, y_val as usize, z_val as usize]) {
                    Some(v) => {
                        if *v == 1 {
                            count_valid += 1;
                        }
                    }
                    None => continue,
                };
            }
        }
    }
    count_valid
}

fn get_padded_array_4_dim(a: &ArrayD<i32>) -> ArrayD<i32> {
    // Return a padded array of the input array for 4 dims
    let new_dim: Vec<usize> = a.dim().as_array_view().iter().map(|x| x + 2).collect();
    let mut b = ArrayD::<i32>::zeros(new_dim);
    b.slice_mut(s![1..-1, 1..-1, 1..-1, 1..-1])
        .assign(&a.slice(s![.., .., .., ..]));
    b
}

fn get_valid_neighbors_4_dims(indexes: &IxDyn, matrix: &ArrayD<i32>) -> i32 {
    // Return the number of active neighbours for 4 dims
    //let (x, y, z, w) = indexes.as_array_view();
    let (x, y, z, w) = (
        indexes.as_array_view()[0],
        indexes.as_array_view()[1],
        indexes.as_array_view()[2],
        indexes.as_array_view()[3],
    );
    let mut count_valid = 0;
    for w_val in w as isize - 1..=w as isize + 1 {
        for z_val in z as isize - 1..=z as isize + 1 {
            for y_val in y as isize - 1..=y as isize + 1 {
                for x_val in x as isize - 1..=x as isize + 1 {
                    if z_val < 0 || y_val < 0 || x_val < 0 || w_val < 0 {
                        // out of bounds
                        continue;
                    }
                    if (x_val, y_val, z_val, w_val)
                        == (x as isize, y as isize, z as isize, w as isize)
                    {
                        // don't count index value itself!
                        continue;
                    }
                    match matrix.get([
                        x_val as usize,
                        y_val as usize,
                        z_val as usize,
                        w_val as usize,
                    ]) {
                        Some(v) => {
                            if *v == 1 {
                                count_valid += 1;
                            }
                        }
                        None => continue,
                    };
                }
            }
        }
    }
    count_valid
}

fn main() {
    let file_path = PathBuf::from("./src/input.txt");
    let f = fs::read_to_string(&file_path).expect("Error reading file");
    let lines: Vec<&str> = f.lines().collect();

    // Part 1 - 3 dims
    // create an nd_array to store the initial values
    let mut active_values = Array3::<i32>::zeros((lines[0].len(), lines.len(), 1));
    for (row_index, line) in lines.iter().enumerate() {
        for (col_index, char) in line.chars().enumerate() {
            match char {
                '#' => {
                    active_values[[col_index, row_index, 0]] = 1;
                }
                '.' => active_values[[col_index, row_index, 0]] = 0,
                _ => panic!("Invalid value"),
            }
        }
    }
    const CYCLES: i32 = 6;

    let mut counter = 0;
    loop {
        if counter == CYCLES {
            break;
        }
        // Pad the original array to iterate the possible neighbors
        let padded_array = get_padded_array(&active_values);

        // Create a new shape with padding to fill the new values
        let mut new_array = Array3::<i32>::zeros(padded_array.dim());

        // Iterate the indexes of the padded array to fill the new matrix
        for (indexes, value) in padded_array.indexed_iter() {
            let num_active_neighbours = get_valid_neighbors(indexes, &padded_array);
            if *value == 1 && ![2, 3].contains(&num_active_neighbours) {
                // value is active and does not contain 2 or 3 active neighbours -> inactive
                new_array[indexes] = 0;
            } else if *value == 0 && num_active_neighbours == 3 {
                // value is inactive and contains 3 active neighbours -> active
                new_array[indexes] = 1;
            } else {
                // value stays the same
                new_array[indexes] = *value;
                if *value > 1 {
                    println!("{}", value)
                }
            }
        }

        active_values = new_array.clone();
        counter += 1;
    }

    let mut count_active = 0;
    for (_, value) in active_values.indexed_iter() {
        if *value == 1 {
            count_active += 1;
        }
    }

    // println!("{:?}", active_values.reversed_axes());
    println!("The number of active cubes for part 1 is {}", count_active);

    // Part 2 - 4 dims
    let shape = IxDyn(&[lines[0].len(), lines.len(), 1, 1]);
    let mut active_values = ArrayD::<i32>::zeros(shape);
    for (row_index, line) in lines.iter().enumerate() {
        for (col_index, char) in line.chars().enumerate() {
            match char {
                '#' => {
                    active_values[[col_index, row_index, 0, 0]] = 1;
                }
                '.' => active_values[[col_index, row_index, 0, 0]] = 0,
                _ => panic!("Invalid value"),
            }
        }
    }

    let mut counter = 0;
    loop {
        if counter == CYCLES {
            break;
        }
        // Pad the original array to iterate the possible neighbors
        let padded_array = get_padded_array_4_dim(&active_values);

        // Create a new shape with padding to fill the new values
        let mut new_array = ArrayD::<i32>::zeros(padded_array.dim());

        // Iterate the indexes of the padded array to fill the new matrix
        for (indexes, value) in padded_array.indexed_iter() {
            let num_active_neighbours = get_valid_neighbors_4_dims(&indexes, &padded_array);
            if *value == 1 && ![2, 3].contains(&num_active_neighbours) {
                // value is active and does not contain 2 or 3 active neighbours -> inactive
                new_array[&indexes] = 0;
            } else if *value == 0 && num_active_neighbours == 3 {
                // value is inactive and contains 3 active neighbours -> active
                new_array[&indexes] = 1;
            } else {
                // value stays the same
                new_array[&indexes] = *value;
                if *value > 1 {
                    println!("{}", value)
                }
            }
        }

        active_values = new_array.clone();
        counter += 1;
    }

    let mut count_active = 0;
    for (_, value) in active_values.indexed_iter() {
        if *value == 1 {
            count_active += 1;
        }
    }

    // println!("{:?}", active_values.reversed_axes());
    println!("The number of active cubes for part 2 is {}", count_active);
}

#[cfg(test)]
mod tests {
    // Import names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_padding() {
        let initial_array = Array3::<i32>::ones((1, 1, 1));
        let padded_array = get_padded_array(&initial_array);
        assert_eq!(padded_array.shape(), [3, 3, 3]);
        assert_eq!(padded_array[(1, 1, 1)], 1);
        assert_eq!(padded_array[(0, 0, 0)], 0);
    }
}
