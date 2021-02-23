// Determine which ingredients can't possibly contain any of the allergens in any food in your list.

use std::fs;
use std::collections::HashMap;
use itertools::Itertools;

fn rules_pass(rule: &(Vec<&str>, Vec<&str>), allergens: &HashMap<String, String>) -> bool {
    // Return true if the current rule works with the proposed allergen values, otherwise return False
    // Check if there is a hard requirement already for an allergen and if not present in the proposed list return false
    // ie. allergens[dairy] = xb, and the rule is ([m], [dairy) -> False
    // while allergens[dairy] = xb, and the rule is ([xb, f, c], [dairy) -> True

    let (rule_ingredients, rule_allergens) = rule;
    for (key, value) in allergens {
        if rule_allergens.contains(&key.as_str()) && !rule_ingredients.contains(&value.as_str()) {
           return false;
        }
    }
    true
}
fn get_allergen_ingredients_permutations(rule: &(Vec<&str>, Vec<&str>), allergens: &HashMap<String, String>) -> Vec<HashMap<String, String>> {
    // Return a list of all possible allergen - ingredients, combinations..?
    // Adds possibilities to the initial dict and returns them
    // This can possibly be improved by doing it recursively

    let used_ingredients: Vec<&String> = allergens.values().collect();
    //let used_ingredients: HashSet<&String>  = used_ingredients.into_iter().collect();
    let used_allergens: Vec<&String> = allergens.keys().collect();
    //let used_allergens: HashSet<&String> = used_allergens.into_iter().collect();

    let (rule_ingredients, rule_allergens) = rule;

    // Todo: do this functionaly instead?
    let mut unused_ingredients = Vec::new();
    for ing in rule_ingredients {
        if !used_ingredients.contains(&&ing.to_string()){
            unused_ingredients.push(*ing);
        }
    }

    let mut unused_allergens = Vec::new();
    for all in rule_allergens {
        if !used_allergens.contains(&&all.to_string()){
            unused_allergens.push(*all);
        }
    }

    // Build all the possible permutations from the leftover allergens and ingredients
    let perms =  unused_ingredients.iter().permutations(unused_allergens.len());
    let mut all_permutation_dicts: Vec<HashMap<String, String>> = Vec::new();
    for perm in perms {
        let zipped: Vec<_> = perm.into_iter().zip(unused_allergens.iter()).collect();
        let mut new_allergens_dict = allergens.clone();
        for values in zipped {
            let (ing, all) = values;
            new_allergens_dict.insert(all.to_string(), ing.to_string());
        }
        all_permutation_dicts.push(new_allergens_dict);
    }

    all_permutation_dicts
}

fn check(rules: &Vec<(Vec<&str>, Vec<&str>)>) -> HashMap<String, String> {
    // Dynamically build a dictionary of ingredients from each line until one is found to work and return it
    fn check_recursive(rules: &Vec<(Vec<&str>, Vec<&str>)>, allergens: &mut HashMap<String, String>) -> Option<HashMap<String, String>> {
        // Return early when the rules have been exhausted, which means it was successful
        if rules.len() == 0 {
            return Some(allergens.clone());
        }
        // Check that the first rule passes
        if !rules_pass(&rules[0], &allergens) {
            return None;
        }

        // Get a list of possible dictionaries containing the possible ingredient and allergens permutations
        let permutations_dicts = get_allergen_ingredients_permutations(&rules[0], &allergens);
        // Todo: simplify this?? Why return final result, return the function instead ?
        for mut permutation_dict in permutations_dicts {
            match check_recursive(&rules[1..].to_vec(), &mut permutation_dict) {
                Some(x) => {
                    return Some(x);
                }
                None => {
                    continue
                }
            }

        }
        None
    }
    let mut allergens: HashMap<String, String> = HashMap::new();
    match check_recursive(rules, &mut allergens) {
        Some(x) => x,
        None => panic!("No result found!"),
    }
}
fn main() {
    let input = fs::read_to_string("./src/input.txt").expect("Error reading file");
    let lines: Vec<&str> = input.lines().collect();

    // Parse into a list of tuples (ingredient, allergens)
    let mut ingredient_allergies: Vec<(Vec<&str>, Vec<&str>)> = Vec::with_capacity(lines.len());

    for line in lines {
        let (left, right) : (&str, &str)= line.splitn(2, " (contains ").collect_tuple().unwrap();
        let ingredients: Vec<&str> = left.split(char::is_whitespace).collect();
        // Remove the last parenthesis
        let right = &right[..right.len() -1];
        let allergens: Vec<&str> = right.split(", ").collect();
        ingredient_allergies.push((ingredients, allergens));
    }

    // Sort by size, to try with the smallest number of combinations first
    //ingredient_allergies.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    ingredient_allergies.sort_by(|a, b| a.1.len().cmp(&b.1.len()));
    println!("ddddd{:?}", ingredient_allergies[0]);
    println!("ddddd{:?}", ingredient_allergies[1]);
    println!("ddddd{:?}", ingredient_allergies[2]);
    // Always assume that one ingredient can be one allergen only, and vice-versa.
    // (ie: if mxmxvkd contains dairy, no other ingredient can contain dairy)

    // Dynamically build a dictionary of ingredients from each line and try to make it work ?
    let matches = check(&ingredient_allergies);

    // Part 1: Determine which ingredients cannot possibly contain any of the allergens in your list.
    // How many times do any of those ingredients appear?
    let all_ingredients :Vec<&String> = matches.values().collect();
    let mut ingredients_without_allergens_sum = 0;
    for (ingredients, _) in ingredient_allergies {
        // get the count of ingredients that are not part of the dict values
        let mut count_in_line = 0;
        for ingredient in ingredients {
            if !all_ingredients.contains(&&ingredient.to_string()) {
                count_in_line += 1;
            }
        }
        ingredients_without_allergens_sum += count_in_line;
    }

    println!("The number of times the ingredients without allergens appear in the list is {}", ingredients_without_allergens_sum);

}

#[cfg(test)]
mod tests {
    // Import names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_contains_allergen() {
        let mut allergens = HashMap::new();

        let rule = (vec!["X", "Y", "M"], vec!["Dairy", "Fish"]);
        // Check that it passes with empty dict
        assert_eq!(rules_pass(&rule, &allergens), true);

        // Check that it passes if an allergen is already defined in the dict and also in the rules
        allergens.insert(
            "Dairy".to_string(),
            "M".to_string(),
        );
        assert_eq!(rules_pass(&rule, &allergens), true);

        // Check that it returns false if an allergen is already defined in the dict but not in the rules
        allergens.insert(
            "Fish".to_string(),
            "N".to_string(),
        );
        assert_eq!(rules_pass(&rule, &allergens), false);
    }

    #[test]
    fn test_permutations() {
        let mut allergens = HashMap::new();
        let rule = (vec!["X", "Y", "Z"], vec!["Dairy", "Fish"]);

        // Check that it returns 6 possible permutations, for an empty dict
        let permutation_dicts = get_allergen_ingredients_permutations(&rule, &allergens);
        assert_eq!(permutation_dicts.len(), 6);

        allergens.insert(
            "Dairy".to_string(),
            "Z".to_string(),
        );
        // Check that it returns 2 possible permutations, with FISH for X and Y and Dairy used for Z
        let permutation_dicts = get_allergen_ingredients_permutations(&rule, &allergens);
        assert_eq!(permutation_dicts.len(), 2);


    }
}