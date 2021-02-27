fn main() {
    // Part 1 - What encryption key is the handshake trying to establish?
    let subject_number = 7;

    // Get card loop size
    let mut card_loop_size = 1;
    let card_public_key = 15628416;
    let mut value = 1;
    loop {
        value = value * subject_number;
        value = value % 20201227;
        if value == card_public_key {
            break;
        }
        card_loop_size += 1;
    }
    println!("Card loop size is {}", card_loop_size);

    // Get door loop size
    let mut door_loop_size = 1;
    let door_public_key = 11161639;
    let mut value = 1;
    loop {
        value = value * subject_number;
        value = value % 20201227;
        if value == door_public_key {
            break;
        }
        door_loop_size += 1;
    }
    println!("Door loop size is {}", door_loop_size);

    // Calculate encryption key by looping x times
    let subject_number = card_public_key;
    let mut loop_size = 1;
    let mut value: usize = 1;
    loop {
        value *= subject_number;
        value %= 20201227;
        if loop_size == door_loop_size {
            break;
        }
        loop_size += 1;
    }
    println!("Encryption key is {}", value);
}
