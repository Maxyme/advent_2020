use std::collections::VecDeque;
use std::thread::current;

fn substract_one(number: usize) -> usize {
    // substract one with wraparound
    if number == 1 {
        return 9;
    }
    number - 1
}



fn main() {
    let mut test_input: Vec<usize> = vec![3, 8, 9, 1, 2, 5, 4, 6, 7];
    let mut puzzle_input: Vec<usize> = vec![3, 8, 9, 1, 2, 5, 4, 6, 7]; //vec![3, 6, 4, 2, 9, 7, 5, 8, 1];
    // What are the labels on the cups after cup 1 after 100 moves;
    let cup_number = 1;
    let mut num_moves = 0; //100;
    let max_moves = 10;
    //let mut previous_destination_before_index = false;
    let mut increment = 0;
    loop {
        if num_moves == max_moves {
            break;
        }
        println!("num_moves: {}, labels {:?}", num_moves + 1, puzzle_input);
        // pick the 3 next to the selected (first) cup

        let mut current_cup_index = num_moves;// % puzzle_input.len();
        // if previous_destination_before_index {
        //     current_cup_index += increment; //3;
        // }
        current_cup_index += increment; //3;
        current_cup_index = current_cup_index % puzzle_input.len();

        let current_cup = puzzle_input[current_cup_index];
        println!("Selected cup: {}", current_cup);

        // need to collect with wrap around
        let start_index = (current_cup_index + 1) % puzzle_input.len();
        let end_index = (current_cup_index + 3) % puzzle_input.len();

        let mut next_cups: Vec<usize> = Vec::new();
        if start_index > end_index {
            // wrap around situation
            let first_batch: Vec<usize> = puzzle_input.drain(start_index..=puzzle_input.len() - 1).collect();
            let last_batch: Vec<usize> = puzzle_input.drain(0..=end_index).collect();

            next_cups = first_batch;
            next_cups.extend(last_batch);

        } else {
            // normal situation
            next_cups = puzzle_input.drain(start_index..=end_index).collect();
        }

        println!("next 3 cups: {:?}", next_cups);

        let destination_cup = {
            let mut next_number = current_cup;
            loop {
                next_number = substract_one(next_number);
                if puzzle_input.contains(&next_number) {
                    break;
                }
            }
            next_number
        };
        println!("destination_cup: {:?}", destination_cup);
        let index_of_destination = puzzle_input.iter().position(|&r| r == destination_cup).expect("Error");
        puzzle_input.splice(index_of_destination + 1..index_of_destination + 1, next_cups);
        num_moves += 1;

        // Increment the index by 3 when removing a slice after the index
        if index_of_destination < current_cup_index {
            //current_cup_index += 3;
            //previous_destination_before_index
            // increment += 3;
            increment = 3;
        } else {
            increment = 0;
        }
        //previous_destination_before_index = index_of_destination < current_cup_index;
    }
    //let destination_cup = substract_one(current_cup);

    // let mut vec = vec![1, 5];
    // let slice = &[2, 3, 4];
    //
    // vec.splice(1..1, slice.iter().cloned());
    //
    // println!("{:?}", vec); // [1, 2, 3, 4, 5]

    //let label_vec = vec![0, 2, 1, 3, 4]; // example vector
    let index_of_one = puzzle_input.iter().position(|&r| r == 1).expect("Error");
    let mut labels_vec = puzzle_input[index_of_one + 1..].to_vec();
    labels_vec.extend_from_slice(&puzzle_input[..index_of_one]); //&label_vec[..index_of_one].to_vec(); //.extend_from_slice(&label_vec[index_of_one..]);
    let labels = labels_vec.into_iter().map(|x| x.to_string()).collect::<Vec<String>>().join("");
    println!("the labels on the cups after cup 1 is: {}", labels)
}
