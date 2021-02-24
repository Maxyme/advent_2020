// Determine which ingredients can't possibly contain any of the allergens in any food in your list.

use itertools::Itertools;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::fs;
use std::hash::Hash;

fn inplace_intersection<T>(a: &mut HashSet<T>, b: &mut HashSet<T>) -> HashSet<T>
where
    T: Hash,
    T: Eq,
{
    // Inplace intersection utility function to return a hashset
    let c: HashSet<T> = a.iter().filter_map(|v| b.take(v)).collect();
    a.retain(|v| !c.contains(&v));
    c
}

fn main() {
    let input = fs::read_to_string("./src/input.txt").expect("Error reading file");
    let lines: Vec<&str> = input.lines().collect();

    // Parse into a list of tuples (ingredient, allergens)
    let mut ingredient_allergies: Vec<(HashSet<&str>, HashSet<&str>)> =
        Vec::with_capacity(lines.len());

    for line in lines {
        let (left, right): (&str, &str) = line.splitn(2, " (contains ").collect_tuple().unwrap();
        let ingredients: HashSet<&str> = left.split(char::is_whitespace).collect();
        // Remove the last parenthesis
        let right = &right[..right.len() - 1];
        let allergens: HashSet<&str> = right.split(", ").collect();
        ingredient_allergies.push((ingredients, allergens));
    }

    // Create a dict containing only the intersections
    // Always assume that one ingredient can be one allergen only, and vice-versa.
    // (ie: if mxmxvkd contains dairy, no other ingredient can contain dairy)
    let mut allergens_dict: HashMap<&str, HashSet<&str>> = HashMap::new();
    for (ingredients, allergies) in &ingredient_allergies {
        for allergy in allergies {
            let mut new_ingredients = ingredients.clone();
            let previous_ingredients = allergens_dict
                .entry(*allergy)
                .or_insert(new_ingredients.clone());
            *previous_ingredients =
                inplace_intersection(previous_ingredients, &mut new_ingredients);
        }
    }

    // Dynamically build a dictionary of ingredients from each line and try to make it work ?
    //let matches = check(&ingredient_allergies);

    // Part 1: Determine which ingredients cannot possibly contain any of the allergens in your list.
    // How many times do any of those ingredients appear?
    // Flatten all the ingredients that are in the allergens dict into a list and check for the lines that don't have these ingredients
    let all_ingredients: Vec<&&str> = allergens_dict.values().flatten().collect::<Vec<&&str>>();
    let mut ingredients_without_allergens_sum = 0;
    for (ingredients, _) in ingredient_allergies {
        // get the count of ingredients that are not part of the dict values
        let mut count_in_line = 0;
        for ingredient in ingredients {
            if !all_ingredients.contains(&&ingredient) {
                count_in_line += 1;
            }
        }
        ingredients_without_allergens_sum += count_in_line;
    }

    println!(
        "The number of times the ingredients without allergens appear in the list is {}",
        ingredients_without_allergens_sum
    );

    // Part 2 sort the elements of the dict by allergens and make a comma separated list of the ingredients
    // Build a BTreeMap (sorted by key) of the ingredients

    // Create a tuple to sort by ingredient count to iterate over after
    let mut allergens_ingredient_tuples: Vec<(&str, HashSet<&str>)> = Vec::new();
    for (allergen, ingredients) in allergens_dict {
        allergens_ingredient_tuples.push((allergen, ingredients));
    }
    allergens_ingredient_tuples.sort_by(|a, b| a.1.len().cmp(&b.1.len()));

    // Create an ordered dict to store the allergen/ingredient
    let mut ordered_allergens: BTreeMap<&str, &str> = BTreeMap::new();
    let mut seen_ingredients: Vec<&str> = Vec::new();
    for (allergen, ingredients) in allergens_ingredient_tuples {
        let ingredient = {
            if ingredients.len() == 1 {
                // Take the first one and add to seen ingredients
                let ingredient = ingredients.iter().next().expect("");
                seen_ingredients.push(ingredient);
                ingredient
            } else {
                // Take the ingredient not in seen ingredients
                ingredients
                    .into_iter()
                    .filter(|x| !seen_ingredients.contains(x))
                    .next()
                    .expect("")
            }
        };
        ordered_allergens.insert(allergen, ingredient);
    }

    println!(
        "The list of canonical dangerous elements is {}",
        ordered_allergens.values().join(",")
    );
}
