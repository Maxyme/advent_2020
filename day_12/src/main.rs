/*
Part 1
The ship starts by facing east. Directions are given with an action and value:

Action N means to move north by the given value.
Action S means to move south by the given value.
Action E means to move east by the given value.
Action W means to move west by the given value.
Action L means to turn left the given number of degrees.
Action R means to turn right the given number of degrees.
Action F means to move forward by the given value in the direction the ship is currently facing.

Return the Manhattan distance between that location and the ship's starting position?

Part 2
Action N means to move the waypoint north by the given value.
Action S means to move the waypoint south by the given value.
Action E means to move the waypoint east by the given value.
Action W means to move the waypoint west by the given value.
Action L means to rotate the waypoint around the ship left (counter-clockwise) the given number of degrees.
Action R means to rotate the waypoint around the ship right (clockwise) the given number of degrees.
Action F means to move forward to the waypoint a number of times equal to the given value.

 */

use std::fs;
use std::path::PathBuf;

fn rotate_value(angle: f64, vec_2d: (i32, i32)) -> (i32, i32) {
    // Rotate vec by value with angle in degrees
    let x = vec_2d.0 * angle.to_radians().cos() as i32 + vec_2d.1 * angle.to_radians().sin() as i32;
    let y =
        -vec_2d.0 * angle.to_radians().sin() as i32 + vec_2d.1 * angle.to_radians().cos() as i32;

    (x, y)
}

fn main() {
    let file_path = PathBuf::from("./src/input.txt"); // use small.txt to practice
    let f = fs::read_to_string(&file_path).expect("Error reading file");
    let lines: Vec<_> = f.lines().collect::<Vec<_>>();

    let mut actions: Vec<(char, i32)> = Vec::with_capacity(lines.len());
    for line in lines {
        let action: char = line.chars().nth(0).expect("Expect int");
        let value: i32 = line[1..].parse::<i32>().expect("Expect int");
        actions.push((action, value));
    }

    // Part 1 - Actions move the ship
    let mut x_pos = 0;
    let mut y_pos = 0;
    let mut current_angle: f64 = 0.0; // 0 is facing starting angle, meaning East
    for (action, value) in &actions {
        match action {
            'N' => y_pos += value,
            'S' => y_pos -= value,
            'E' => x_pos += value,
            'W' => x_pos -= value,
            'L' => current_angle -= *value as f64,
            'R' => current_angle += *value as f64,
            'F' => {
                x_pos += current_angle.to_radians().cos() as i32 * value;
                y_pos -= current_angle.to_radians().sin() as i32 * value;
            }
            _ => panic!("This is not supposed to happen"),
        }
    }
    println!(
        "The manhattan distance for part 1 is {}",
        x_pos.abs() + y_pos.abs()
    );

    // Part 2 - Actions move a waypoint where the ship is going
    // waypoint is 10 units east and 1 unit north
    let mut x_pos = 0;
    let mut y_pos = 0;
    let mut x_pos_wp = 10;
    let mut y_pos_wp = 1;
    for (action, value) in &actions {
        match action {
            'N' => y_pos_wp += value,
            'S' => y_pos_wp -= value,
            'E' => x_pos_wp += value,
            'W' => x_pos_wp -= value,
            'L' => {
                // Note latest nightly version can assign tuple to mut ie: (x_pos_wp, y_pos_wp) = rotate_value..
                let (x, y) = rotate_value(-*value as f64, (x_pos_wp, y_pos_wp));
                x_pos_wp = x;
                y_pos_wp = y;
            }
            'R' => {
                let (x, y) = rotate_value(*value as f64, (x_pos_wp, y_pos_wp));
                x_pos_wp = x;
                y_pos_wp = y;
            }
            'F' => {
                x_pos += value * x_pos_wp;
                y_pos += value * y_pos_wp;
            }
            _ => panic!("This is not supposed to happen"),
        }
    }
    println!(
        "The manhattan distance for part 2 is {}",
        x_pos.abs() + y_pos.abs()
    );
}
