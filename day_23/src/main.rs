fn subtract_one(number: usize, max_input: usize) -> usize {
    // Subtract one with wraparound
    if number == 1 {
        return max_input;
    }
    number - 1
}

fn game(input: &Vec<usize>, max_moves: &usize) -> Vec<usize> {
    // Brute force version of the game
    let mut num_moves = 0;
    let max_num = input.len();
    let mut puzzle_input: Vec<usize> = input.clone();
    loop {
        if &num_moves == max_moves {
            break;
        }

        println!("num_moves: {}", num_moves);
        let current_cup_index = num_moves % puzzle_input.len();
        let current_cup = puzzle_input[current_cup_index];

        println!("Selected cup: {}", current_cup);

        let next_cups: Vec<usize> = {
            // Collect the next cups with wrap around
            let start_index = (current_cup_index + 1) % puzzle_input.len();
            let end_index = (current_cup_index + 3) % puzzle_input.len();
            let mut next_cups;
            if start_index > end_index {
                // wrap around situation
                let first_batch: Vec<usize> = puzzle_input
                    .drain(start_index..=puzzle_input.len() - 1)
                    .collect();
                let last_batch: Vec<usize> = puzzle_input.drain(0..=end_index).collect();
                next_cups = first_batch;
                next_cups.extend(last_batch);
            } else {
                // normal situation
                next_cups = puzzle_input.drain(start_index..=end_index).collect();
            }
            next_cups
        };
        println!("next 3 cups: {:?}", next_cups);

        let destination_cup_index = {
            let mut next_number = current_cup;
            loop {
                next_number = subtract_one(next_number, max_num);
                if !next_cups.contains(&next_number) {
                    break;
                }
            }
            println!("destination_cup: {:?}", next_number);
            puzzle_input
                .iter()
                .position(|&r| r == next_number)
                .expect("Error number not found")
        };

        puzzle_input.splice(
            destination_cup_index + 1..destination_cup_index + 1,
            next_cups,
        );

        // rotate vec so the current cup is located at the index after the splice
        let new_current_cup_index = puzzle_input
            .iter()
            .position(|&r| r == current_cup)
            .expect("Error");

        if new_current_cup_index > current_cup_index {
            let rotate = new_current_cup_index - current_cup_index;
            puzzle_input.rotate_left(rotate);
        } else if new_current_cup_index < current_cup_index {
            let rotate = current_cup_index - new_current_cup_index;
            puzzle_input.rotate_right(rotate);
        }

        num_moves += 1;
    }
    puzzle_input.clone()
}

fn game_with_indexed_list(input: &Vec<usize>, current_cup: usize, max_moves: &usize) -> Vec<usize> {
    // Run the loop on an indexed list, meaning each index represents a cup,
    // and the value at the index points to the next cup clockwise
    let mut num_moves = 0;
    let mut current_cup = current_cup;
    let max_num = input.len() - 1;

    let mut indexed_list: Vec<usize> = input.clone();
    loop {
        if &num_moves == max_moves {
            break;
        }

        // get the next 3 cups clockwise
        let first_cup = indexed_list[current_cup];
        let second_cup = indexed_list[first_cup];
        let third_cup = indexed_list[second_cup];

        // make the first and second cups points to each other
        indexed_list[first_cup] = second_cup;
        indexed_list[second_cup] = third_cup;

        // Save the current cup to point to the one the third cup is pointing to, for the next round
        indexed_list[current_cup] = indexed_list[third_cup];

        // get destination cup
        let destination_cup = {
            let mut next_number = current_cup;
            loop {
                next_number = subtract_one(next_number, max_num);
                if ![first_cup, second_cup, third_cup].contains(&next_number) {
                    break;
                }
            }
            next_number
        };

        // set the third cup to point to where the destination cup is pointing
        indexed_list[third_cup] = indexed_list[destination_cup];

        // then set the destination cup to point to the first of 3 displaced cups
        indexed_list[destination_cup] = first_cup;

        // switch the current cup to the next cup
        current_cup = indexed_list[current_cup];
        num_moves += 1;
    }

    indexed_list.clone()
}

fn make_1_based_indexed_list(input: &Vec<usize>) -> Vec<usize> {
    // return a 1-based indexed vec of the given vec. Add 1 to account for 0 which points to itself
    let mut output: Vec<usize> = vec![0; input.len() + 1];
    output[0] = 0;
    for (index, value) in input.iter().enumerate() {
        if index == input.len() - 1 {
            output[*value] = input[0];
        } else {
            output[*value] = input[index + 1];
        }
    }
    output
}

fn main() {
    let mut puzzle_input: Vec<usize> = vec![3, 6, 4, 2, 9, 7, 5, 8, 1];

    // Part 1 - What are the labels on the cups after cup 1 after 100 moves (this one can be brute forced);
    let max_moves = 100;
    let final_cup_placement = game(&mut puzzle_input, &max_moves);

    let index_of_one = final_cup_placement
        .iter()
        .position(|&r| r == 1)
        .expect("Error");
    let mut labels_vec = final_cup_placement[index_of_one + 1..].to_vec();
    labels_vec.extend_from_slice(&final_cup_placement[..index_of_one]);
    let labels = labels_vec
        .into_iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join("");
    println!("The labels on the cups after cup 1 are: {}", labels);

    // Part 2 - Determine which two cups will end up immediately clockwise of cup 1.
    // What do you get if you multiply their labels together?

    // Make a list where the index is the cup value, and the value is the next cup (ie: a linked-list)
    // with 0 pointing to itself, for 1-based indexing
    let indexed_input = make_1_based_indexed_list(&puzzle_input);

    // create the million length vector by splicing in the indexed input
    let mut million_vec: Vec<usize> = (1..=1_000_001).collect();
    million_vec.splice(..indexed_input.len(), indexed_input.clone());

    // connect the last value of the input array to the next value of the million item array
    let last_value = puzzle_input.last().expect("Error getting last value");
    million_vec[*last_value] = indexed_input.len();

    // connect the last value of the bigger array to the first value of smaller array
    let last_item = million_vec.last_mut().expect("Error getting last item");
    *last_item = *puzzle_input.first().expect("Error getting first value");

    let ten_million_moves = 10_000_000;
    let final_indexed_list =
        game_with_indexed_list(&million_vec, puzzle_input[0], &ten_million_moves);

    let first_cup_after_one = final_indexed_list[1];
    let second_cup_after_one = final_indexed_list[first_cup_after_one];
    println!(
        "The labels after cup 1 are {} and {}. Multiplied together: {}",
        first_cup_after_one,
        second_cup_after_one,
        first_cup_after_one * second_cup_after_one
    );
}
