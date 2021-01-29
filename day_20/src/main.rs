/*
Each camera in the camera array returns a single monochrome image tile with a random unique ID number.
The tiles (your puzzle input) arrived in a random order.
Each image tile has been rotated and flipped to a random orientation, and each tile's image data includes a border that should line up exactly with its adjacent tiles.
 */

use itertools::Itertools;
use ndarray::{s, Array2};
use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

const ROTATIONS: [usize; 4] = [0, 90, 180, 270];

#[derive(Debug, Clone)]
struct Tile {
    id: usize,
    data: Array2<usize>,
}

fn get_all_permutations(array: &Array2<usize>) -> Vec<Array2<usize>> {
    // Return a list of all the possible permutations. Todo. make into a generator?
    let mut permutations: Vec<Array2<usize>> = Vec::new();
    for flip_image in [false, true].iter() {
        for rotation in ROTATIONS.iter() {
            // Flip and rotate candidate array .todo: make this faster?
            let new_array = {
                let mut new_array = array.to_owned();
                if *flip_image {
                    //new_array = flip_array(&new_array);
                    // Flip an array horizontally
                    new_array = array.slice(s![.., ..; -1]).to_owned();
                }
                if [90, 180, 270].contains(rotation) {
                    new_array = rotate_array(&new_array, *rotation);
                }
                new_array
            };
            permutations.push(new_array);
        }
    }
    permutations
}

fn rotate_array(array: &Array2<usize>, rotation: usize) -> Array2<usize> {
    // Rotate an image by 90 degrees, x times

    fn rotate_90(array: &Array2<usize>) -> Array2<usize> {
        // reverse each column then transpose
        array.slice(s![.., ..; -1]).t().into_owned()
    }

    let mut b = array.clone();
    let mut angle = 0;
    loop {
        if angle == rotation {
            break;
        }
        b = rotate_90(&b);
        angle += 90;
    }
    b
}

fn get_array_from_lines(lines: &Vec<&str>) -> Array2<usize> {
    // create a matrix from the given lines
    let mut array = Array2::<usize>::zeros((lines.len(), lines[0].len()));
    for (index, line) in lines.iter().enumerate() {
        // Convert # to 1 and . to 0
        for (char_index, char) in line.chars().enumerate() {
            let value = {
                match char {
                    '#' => 1,
                    _ => 0,
                }
            };
            array.row_mut(index)[char_index] = value;
        }
    }
    array
}

fn get_tiles_dict(input_entries: &Vec<&str>) -> HashMap<usize, Array2<usize>> {
    // create a dict with id and tiles values
    let mut images_dict: HashMap<usize, Array2<usize>> = HashMap::new();
    for entry in input_entries {
        let first_line: &str = entry.lines().next().expect("Error getting tile ID");
        let id: usize = first_line
            .chars()
            .filter(|x| x.is_numeric())
            .collect::<String>()
            .parse()
            .expect("Error parsing ID");

        let value_lines = entry.lines().collect_vec();
        let array = get_array_from_lines(&value_lines[1..].to_vec());
        images_dict.insert(id, array);
    }
    images_dict
}

fn can_fit_with_rotation(
    new_array: &Array2<usize>,
    current_tiles: &Vec<Tile>,
    image_side: usize,
) -> bool {
    // Return true if the tile edges can be fit in the image with the given rotation, otherwise false

    let tile_is_on_top = current_tiles.len() < image_side;
    let match_top = {
        if tile_is_on_top {
            // no neighbors to match if on top row
            true
        } else {
            // compare new_array.top == top_neighbor_tile.bottom
            let top_neighbor_tile = &current_tiles[current_tiles.len() - image_side];
            &new_array.row(0)
                == &top_neighbor_tile
                    .data
                    .row(top_neighbor_tile.data.nrows() - 1)
        }
    };
    let tile_is_on_left = current_tiles.len() % image_side == 0;
    let match_left = {
        if tile_is_on_left {
            // no neighbors to match if on left column
            true
        } else {
            // compare to new_array.left == left_neighbor_tile.right
            let left_neighbor_tile = &current_tiles[current_tiles.len() - 1];
            &new_array.column(0)
                == &left_neighbor_tile
                    .data
                    .column(&left_neighbor_tile.data.ncols() - 1)
        }
    };
    match_left && match_top
}

fn match_tiles(tiles: &Vec<Tile>) -> Vec<Tile> {
    fn match_tiles_recursive(
        candidates: &Vec<Tile>,
        placed_tiles: &Vec<Tile>,
        image_side: usize,
    ) -> Option<Vec<Tile>> {
        if candidates.len() == 0 {
            // Return early if no candidates
            Some(placed_tiles.to_vec())
        } else {
            for candidate in candidates {
                for possible_permutation in get_all_permutations(&candidate.data).iter() {
                    if can_fit_with_rotation(&possible_permutation, placed_tiles, image_side) {
                        // Remove candidate from new list. todo: make this simpler
                        let mut new_candidates = candidates.clone();
                        let index = new_candidates
                            .iter()
                            .position(|x| x.id == candidate.id)
                            .unwrap();
                        new_candidates.remove(index);

                        // Add candidate to possible image
                        let tile = Tile {
                            id: candidate.id,
                            data: possible_permutation.to_owned(),
                        };

                        let mut tiles = placed_tiles.clone();
                        tiles.push(tile);

                        match match_tiles_recursive(&new_candidates, &tiles, image_side) {
                            Some(x) => return Some(x),
                            None => continue,
                        }
                    }
                }
            }
            None
        }
    }
    let mut image: Vec<Tile> = Vec::with_capacity(tiles.len());
    let image_side = (tiles.len() as f64).sqrt() as usize;
    match match_tiles_recursive(tiles, &mut image, image_side) {
        Some(x) => x,
        None => panic!("No result found!"),
    }
}

fn get_pattern_count(image_2d: &Array2<usize>, pattern: &Array2<usize>) -> usize {
    // Return the number of times the pattern has been found in the image
    // Note: the spaces (0s) can be anything; only the # (1s) need to match

    fn slice_pattern_match(slice: &Array2<usize>, pattern: &Array2<usize>) -> bool {
        // check if the pattern matches the slice where indexes of 1s in the pattern are present in the slice
        // todo: filter where values are 1s, to make a smaller list
        for (index, value) in pattern.indexed_iter() {
            if value == &1 && slice[(index)] != 1 {
                return false;
            }
        }
        return true;
    }

    let mut count = 0;
    for (index, _) in image_2d.indexed_iter() {
        if index.0 > image_2d.dim().1 - pattern.dim().1
            || index.1 > image_2d.dim().0 - pattern.dim().0
        {
            // don't check if the pattern would not fit
            continue;
        }
        let slice = image_2d.slice(s![
            index.1..index.1 + pattern.dim().0,
            index.0..index.0 + pattern.dim().1
        ]);
        if slice_pattern_match(&slice.to_owned(), &pattern) {
            count += 1;
        }
    }
    count
}

fn main() {
    let file_path = PathBuf::from("./src/input.txt");
    let f = fs::read_to_string(&file_path).expect("Error reading file");
    let input_entries: Vec<&str> = Regex::new(r"\n\n").unwrap().split(&f).collect::<Vec<_>>(); // split on empty lines

    // Create dict with the image data
    let tiles_dict = get_tiles_dict(&input_entries);

    // Get the image vector of rotated tiles
    let mut candidates: Vec<Tile> = Vec::with_capacity(tiles_dict.len());
    for (tile_id, array) in tiles_dict.iter() {
        candidates.push(Tile {
            id: *tile_id,
            data: array.to_owned(),
        })
    }
    let image = match_tiles(&candidates);

    // Part 1: Multiply the IDs of the four corner tiles together once assembled
    let image_side_count = (candidates.len() as f64).sqrt() as usize;
    let sum = &image[0].id
        * &image[image.len() - 1].id
        * &image[&image_side_count - 1].id
        * &image[image.len() - &image_side_count].id;
    println!("The sum of all four corner ids is {}", sum);

    // Part 2: find the pattern in the composite image
    // Read the pattern into a 2d array
    let pattern_path = PathBuf::from("./src/pattern.txt");
    let f = fs::read_to_string(&pattern_path).expect("Error reading file");
    let pattern_lines: Vec<&str> = f.lines().collect();
    let pattern = get_array_from_lines(&pattern_lines);

    // Create an image from the tiles by removing the edges of each tile to form the actual image:
    let sliced_tile_len = candidates[0].data.row(0).len() - 2;
    let side_len = sliced_tile_len * &image_side_count;
    let mut composed_image = Array2::<usize>::zeros((side_len, side_len));
    for (index, tile) in image.iter().enumerate() {
        let row_start = (index % &image_side_count) * sliced_tile_len;
        let row_end = row_start + sliced_tile_len;
        let column_start = (index / &image_side_count) * sliced_tile_len;
        let column_end = column_start + sliced_tile_len;
        // Remove borders from slice
        let slice = &tile.data.slice(s![1..-1, 1..-1]);
        // Add to composed image
        composed_image
            .slice_mut(s![column_start..column_end, row_start..row_end])
            .assign(&slice);
    }

    // Try all permutations possible to see if the pattern is present more than 0 times in one permutation
    let mut pattern_count = 0;
    let possible_images = get_all_permutations(&composed_image);
    for possible_image in possible_images {
        pattern_count = get_pattern_count(&possible_image, &pattern);
        if pattern_count > 0 {
            break;
        }
    }

    // Get the sum of all hashes when removing the number of hashes found in the patterns
    let pattern_hashes = pattern_count * pattern.sum();
    let hash_sum = composed_image.sum() - pattern_hashes;
    println!(
        "The num of # that are not part of sea monsters is {}",
        hash_sum
    );
}

#[cfg(test)]
mod tests {
    // Import names from outer (for mod tests) scope.
    use super::*;
    use ndarray::arr2;

    #[test]
    fn test_rotate_array() {
        let array = arr2(&[[1, 2, 3], [4, 5, 6], [7, 8, 9]]);
        let rotated_90 = rotate_array(&array, 90);
        assert_eq!(rotated_90.row(0).to_vec(), vec![3, 6, 9]);
        let rotated_180 = rotate_array(&array, 180);
        assert_eq!(rotated_180.row(0).to_vec(), vec![9, 8, 7]);
    }

    #[test]
    fn test_pattern_count_1() {
        let array = arr2(&[[1, 0, 1], [1, 1, 1], [1, 0, 1]]);
        let pattern = arr2(&[[1, 0, 1]]);
        let pattern_count = get_pattern_count(&array, &pattern);
        assert_eq!(pattern_count, 3);
    }

    #[test]
    fn test_pattern_count_2() {
        // Test with a pattern over multiple rows
        let array = arr2(&[[1, 0, 1], [1, 1, 1], [1, 0, 1]]);
        let pattern = arr2(&[[1, 0, 1], [1, 1, 1]]);
        let pattern_count = get_pattern_count(&array, &pattern);
        assert_eq!(pattern_count, 1);
        let array = arr2(&[[1, 0, 1], [1, 1, 1], [1, 0, 1]]);
        let pattern = arr2(&[[1, 0, 1], [1, 0, 1]]);
        let pattern_count = get_pattern_count(&array, &pattern);
        assert_eq!(pattern_count, 2);
    }

    #[test]
    fn test_pattern_count_3() {
        // Test with an array that needs to be rotated
        let array = arr2(&[[1, 0, 1], [1, 0, 1], [1, 0, 1]]);
        let pattern = arr2(&[[1, 1, 1], [0, 0, 0]]);
        let possible_images = get_all_permutations(&array);
        let mut pattern_count = 0;
        for possible_image in possible_images {
            pattern_count = get_pattern_count(&possible_image, &pattern);
            if pattern_count > 0 {
                println!("{:?}", possible_image);
                break;
            }
        }
        assert_eq!(pattern_count, 1);
    }
}
