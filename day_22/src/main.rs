// Day 22 - Calculate the winning player's score in a combat card game

use regex::Regex;
use std::collections::VecDeque;
use std::fs;

//fn recursive_combat(deck_1: &mut VecDeque<usize>, deck_2: &mut VecDeque<usize>) -> (VecDeque<usize>, VecDeque<usize>) {
fn recursive_combat<'a>(
    deck_1: &'a mut VecDeque<usize>,
    deck_2: &'a mut VecDeque<usize>,
) -> Vec<&'a usize> {
    // Recursively peform a game of combat with added rules

    // fn recursive_combat<'a>(deck_1: &'a mut VecDeque<usize>, deck_2: &'a mut VecDeque<usize>) -> Vec<&'a usize> {
    //
    // }
    //
    if deck_1.len() > deck_2.len() {
        deck_1.iter().collect()
    } else {
        deck_2.iter().collect()
    }
}

fn combat<'a>(deck_1: &'a mut VecDeque<usize>, deck_2: &'a mut VecDeque<usize>) -> Vec<&'a usize> {
    // loop until a deck is empty
    loop {
        if deck_1.len() == 0 || deck_2.len() == 0 {
            break;
        }
        // pick the first one one top and compare
        let card_1 = deck_1.pop_front().expect("Value Error");
        let card_2 = deck_2.pop_front().expect("Value Error");
        if card_1 > card_2 {
            deck_1.push_back(card_1);
            deck_1.push_back(card_2);
        } else {
            deck_2.push_back(card_2);
            deck_2.push_back(card_1);
        }
    }

    // Return winning deck
    if deck_1.len() > deck_2.len() {
        deck_1.iter().collect()
    } else {
        deck_2.iter().collect()
    }
}

fn get_best_deck_score(deck: &Vec<&usize>) -> usize {
    // Multiply the value of the cards by it's reverse index starting at one to get the sum
    let mut sum = 0;
    for (index, value) in deck.iter().enumerate() {
        sum += *value * (deck.len() - index);
    }
    sum
}

fn main() {
    let input = fs::read_to_string("./src/input.txt").expect("Error reading file");

    // split on empty lines
    let input_entries: Vec<&str> = Regex::new(r"\n\n")
        .unwrap()
        .split(&input)
        .collect::<Vec<_>>();

    // Parse the values into usize
    let parse_values = |entries: &str| -> VecDeque<usize> {
        entries
            .lines()
            .skip(1)
            .map(|x| x.parse::<usize>().expect("Error parsing value into usize"))
            .collect::<VecDeque<usize>>()
    };
    let mut player_1_deck = parse_values(input_entries[0]);
    let mut player_2_deck = parse_values(input_entries[1]);

    // Part 2 - normal combat
    let best_deck = combat(&mut player_1_deck, &mut player_2_deck);

    // Calculate the winning player's score
    let score = get_best_deck_score(&best_deck);
    println!("The winning player's score for part 1 is {}", score);

    // Part 2 - Recursive combat

    // Add 3 rules to the previous rules
    // 1 - If there was a previous round in this game that had exactly the same cards in the same order in the same players' decks, the game instantly ends in a win for player 1

    // 2 - If both players have at least as many cards remaining in their deck as the value of the card they just drew, the winner of the round is determined by playing a new game of Recursive Combat
    // ie, player 1 draws 4 and has a deck size of at least 4 cards, and the same for player 2

    // 3 - If at least one player doesn't have enough cards left in their deck to recurse; the winner of the round is the player with the higher-value card
    let mut player_1_deck = parse_values(input_entries[0]);
    let mut player_2_deck = parse_values(input_entries[1]);
    let best_deck = recursive_combat(&mut player_1_deck, &mut player_2_deck);

    // Calculate the winning player's score
    let score = get_best_deck_score(&best_deck);
    println!("The winning player's score for part 2 is {}", score);
}
