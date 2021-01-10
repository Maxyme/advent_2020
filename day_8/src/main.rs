/* Get the value in the executor before any command is executed again

acc increases or decreases a single global value called the accumulator by the value given in the argument.
For example, acc +7 would increase the accumulator by 7. The accumulator starts at 0.
After an acc instruction, the instruction immediately below it is executed next.

jmp jumps to a new instruction relative to itself. The next instruction to execute is found using the argument as an offset from the jmp instruction;
for example, jmp +2 would skip the next instruction, jmp +1 would continue to the instruction immediately below it,
and jmp -20 would cause the instruction 20 lines above to be executed next.

nop stands for No OPeration - it does nothing. The instruction immediately below it is executed next.
 */

use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::PathBuf;

fn perform_cmd(index: i32, cmd: &str, value: i32) -> (i32, i32) {
    // perform the command at the index and return the new index
    match cmd {
        "acc" => return (index + 1, value),
        "jmp" => {
            return (index + value, 0);
        }
        "nop" => return (index + 1, 0),
        _ => panic!("This is not supposed to happen"),
    }
}

fn run(commands: &HashMap<i32, (&str, i32)>) -> (i32, bool) {
    let mut index: i32 = 0;
    let mut aggregator: i32 = 0;
    let mut finished = false;
    let mut visited_indexes = HashSet::<i32>::new();

    // Loop until the index value has been seen in the hashset or until end is reached
    loop {
        let (cmd, value) = commands[&index];
        let (new_index, agg_value_increase) = perform_cmd(index, cmd, value);
        index = new_index;
        aggregator += agg_value_increase;
        if visited_indexes.contains(&index) {
            println!("Stuck in an infinite loop at index {}", index);
            break;
        } else if index == commands.len() as i32 {
            println!("Finished processing commands");
            finished = true;
            break;
        }
        visited_indexes.insert(index);
    }
    (aggregator, finished)
}

fn main() {
    let file_path = PathBuf::from("./src/input.txt");
    let f = fs::read_to_string(&file_path).expect("Error reading file");
    let input_lines: Vec<_> = f.lines().collect::<Vec<_>>(); // split on empty lines

    // build command dictionary
    let mut command_dict = HashMap::<i32, (&str, i32)>::new();
    for (index, cmd) in input_lines.iter().enumerate() {
        let (command, value) = cmd.splitn(2, char::is_whitespace).collect_tuple().unwrap();
        let value = value.parse::<i32>().unwrap();
        command_dict.insert(index as i32, (command, value));
    }

    // Part 1 - find the aggregator value when it crashes
    let (aggregator, finished) = run(&command_dict);
    println!("The aggregator value for part 1 is {}", aggregator);

    // Part 2 - brute force try changing one instruction at a time to see if it ever finishes
    for (index, cmd) in command_dict.iter() {
        let mut updated_command_dict = command_dict.clone();
        if cmd.o == "agg" {
            // skip agg values
            continue;
        }
        if cmd.0 == "nop" {
            updated_command_dict.insert(*index, ("jmp", cmd.1));
        } else if cmd.0 == "jmp" {
            updated_command_dict.insert(*index, ("nop", cmd.1));
        }
        let (aggregator, finished) = run(&updated_command_dict);
        if finished == true {
            println!("The aggregator value for part 2 is {}", aggregator);
            break;
        }
    }
}
