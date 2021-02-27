// Follow instructions to flip tiles in a hex-grid
// 6 directions from the center tile are e - se - sw - w - nw - ne

use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs;

fn get_vector_from_line(line: &str) -> (isize, isize) {
    // Return a direction vector based on the rules
    // split the rule in elements of e, se, sw, w, nw, and ne
    let re = Regex::new(r"(?:ne|se|e|nw|sw|w)").expect("Regex invalid");
    let dirs: Vec<&str> = re.find_iter(line).map(|mat| mat.as_str()).collect();

    // update the final vector with
    let mut final_vector: (isize, isize) = (0, 0);
    for dir in dirs {
        match dir {
            "e" => final_vector.0 += 2,
            "w" => final_vector.0 -= 2,
            "ne" => {
                final_vector.0 += 1;
                final_vector.1 += 1;
            }
            "se" => {
                final_vector.0 += 1;
                final_vector.1 -= 1;
            }
            "nw" => {
                final_vector.0 -= 1;
                final_vector.1 += 1;
            }
            "sw" => {
                final_vector.0 -= 1;
                final_vector.1 -= 1;
            }
            _ => panic!("unexpected value"),
        }
    }
    final_vector
}

fn get_all_neighbors(tile: (isize, isize)) -> Vec<(isize, isize)> {
    // Return a list of 6 neighbors for the given tile
    let mut neighbors: Vec<(isize, isize)> = Vec::with_capacity(6);
    neighbors.push((tile.0 + 2, tile.1));
    neighbors.push((tile.0 - 2, tile.1));
    neighbors.push((tile.0 + 1, tile.1 + 1));
    neighbors.push((tile.0 + 1, tile.1 - 1));
    neighbors.push((tile.0 - 1, tile.1 + 1));
    neighbors.push((tile.0 - 1, tile.1 - 1));
    neighbors
}

fn get_count_black_tile(
    tiles: &Vec<(isize, isize)>,
    dict: &HashMap<(isize, isize), usize>,
) -> usize {
    let mut count = 0;
    for tile in tiles {
        // if the dict doesn't contain the tile, it's expected to be white
        match dict.get(&tile) {
            Some(color) => {
                if color == &0 {
                    count += 1;
                }
            }
            None => continue,
        }
    }
    count
}

fn main() {
    // Read each instruction in a vec
    let input = fs::read_to_string("./src/input.txt").expect("Error reading file");
    let input_lines: Vec<&str> = input.lines().collect();

    // Create a dict of tile directions from 0,0 and update the corresponding tile colors
    let mut tiles_colors: HashMap<(isize, isize), usize> = HashMap::new();
    for line in input_lines {
        let final_vector: (isize, isize) = get_vector_from_line(line);

        // Add or update (flip the color) the dictionary of tiles with color values
        let color = tiles_colors.entry(final_vector).or_insert(1);
        *color = 1 - *color;
    }

    // Part 1 - how many tiles are left with the black side up?
    let black_tiles_count: usize = tiles_colors.values().filter(|x| x == &&0).count();
    println!(
        "The number of black tiles for part 1 is: {}",
        black_tiles_count
    );

    // Part 2 - Tiles flip every day according to 2 rules:
    // 1 - Any black tile with zero or more than 2 black tiles immediately adjacent to it is flipped to white.
    // 2 - Any white tile with exactly 2 black tiles immediately adjacent to it is flipped to black.
    let total_days = 100;
    let mut num_days = 0;
    loop {
        if num_days == total_days {
            break;
        }
        // create a list of tiles to flip
        let mut flips: Vec<(isize, isize)> = Vec::new();
        // create a hashset of white tiles to flip. Hashset because they might be added more than once
        let mut white_tiles_to_flip: HashSet<(isize, isize)> = HashSet::<(isize, isize)>::new();
        for (tile, color) in tiles_colors.iter() {
            // if the tile color is black, get the number of black adjacent tiles
            if color != &0 {
                // skip if white
                continue;
            }

            let six_neighbors: Vec<(isize, isize)> = get_all_neighbors(*tile); // should return a list of 6 neighbors
            let black_adjacent_tiles_count = get_count_black_tile(&six_neighbors, &tiles_colors); //get_all_adjacecent_in_dict...
            if black_adjacent_tiles_count == 0 || black_adjacent_tiles_count > 2 {
                flips.push(*tile);
            }
            // get all possible neighbors
            for neighbor in six_neighbors.iter() {
                let six_neighbors: Vec<(isize, isize)> = get_all_neighbors(*neighbor);
                let black_adjacent_tiles_count =
                    get_count_black_tile(&six_neighbors, &tiles_colors);

                let tile_color_is_white = {
                    match tiles_colors.get(&neighbor) {
                        Some(color) => color != &0,
                        None => true,
                    }
                };
                if tile_color_is_white && black_adjacent_tiles_count == 2 {
                    white_tiles_to_flip.insert(*neighbor);
                }
            }
        }

        // Add the hashset to the list of flips
        let mut white_flips: Vec<(isize, isize)> = white_tiles_to_flip.into_iter().collect();
        flips.append(&mut white_flips);
        // execute the flips
        for flip in flips {
            let color = tiles_colors.entry(flip).or_insert(1);
            *color = 1 - *color;
        }
        num_days += 1;
    }

    // How many tiles will be black after 100 days?
    let black_tiles_count: usize = tiles_colors.values().filter(|x| x == &&0).count();
    println!(
        "The number of black tiles for part 2 is: {}",
        black_tiles_count
    );
}
