// Determine which ingredients can't possibly contain any of the allergens in any food in your list.

use std::fs;
use std::collections::HashMap;
use itertools::Itertools;
use std::collections::hash_map::RandomState;

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

// fn get_allergen_ingredients_permutations_recursively(rule: &(Vec<&str>, Vec<&str>), allergens: &HashMap<String, String>) -> Vec<HashMap<String, String>> {
//
//
// }

fn get_allergen_ingredients_permutations(rule: &(Vec<&str>, Vec<&str>), allergens: &HashMap<String, String>) -> Vec<HashMap<String, String>> {
    // Return a list of all possible allergen - ingredients, combinations..?
    // Adds possibilities to the initial dict and returns them
    // This can possibly be improved by doing it recursively
    let mut dicts: Vec<HashMap<String, String>> = Vec::new();


    let (rule_ingredients, rule_allergens) = rule;
    // let it = rule_allergens.iter().cartesian_product(rule_ingredients);
    // for item in it {
    //     println!("{:?}", item);
    // }


    let perms =  rule_allergens.iter().permutations(rule_allergens.len());
    // for each_permutation in list1_permutations:
    //     zipped = zip(each_permutation, list2)
    //     all_combinations.append(list(zipped))
    let mut all_combinations: Vec<(Vec<&str>, Vec<&str>)> = Vec::new();
    for perm in perms {
        //let mut iter = a1.iter().zip(a2.iter());
        let mut zipped = perm.iter().zip(rule_ingredients.iter());
        let values: (&&&str, &&str) = zipped.next().expect(""); //.collect();
        println!("{:?}", values)
        //all_combinations.push(values);
    }

    // for item in all_combinations {
    //     println!("{:?}", item);
    // }

    //itertools::assert_equal(it, vec![(0, 'α'), (0, 'β'), (1, 'α'), (1, 'β')]);
    // Check >?? fn combinations_with_replacement(self, k: usize) -> CombinationsWithReplacement<Self>

    let used_ingredients: Vec<&String> = allergens.values().collect();

    for rule_allergen in rule_allergens {
        match allergens.get(&*rule_allergen.to_string()) {
            Some(x) => {
                // skip if already present in dict
                continue
            },
            None => {
                let mut possible_dict = allergens.clone();
                for ingredient  in rule_ingredients {
                    if !used_ingredients.contains(&&ingredient.to_string()) {
                        possible_dict.insert(
                            rule_allergen.to_string(),
                            ingredient.to_string(),
                        );
                        // dicts.push(possible_dict.to_owned());
                    }
                }
                dicts.push(possible_dict.to_owned());
            }
        }
    }
    dicts
    // for rule_allergen in rule_allergens {
    //     let mut possible_dict = allergens.clone();
    //     // insert a key only if it doesn't already exist
    //     possible_dict.entry(rule_allergen).or_insert(100);
    //     // match allergens.get(book) {
    //     //     Some(x) => continue,
    //     //     None => println!("{} is unreviewed.", book)
    //     // }
    // }
    // for (allergen, ingredient) in iproduct!(0..4, 0..4, 0..4) {
    //     // ..
    // }
    // for
}

fn check(rules: &Vec<(Vec<&str>, Vec<&str>)>) -> HashMap<String, String> {
    // Dynamically build a dictionary of ingredients from each line until one is found to work and return it
    fn check_recursive(rules: &Vec<(Vec<&str>, Vec<&str>)>, allergens: &mut HashMap<String, String>) -> Option<HashMap<String, String>> {
        // return early when the rules have been exhausted
        if rules.len() == 0 {
            return Some(allergens.clone());
        }
        // Update the dict with values from the rule
        // let (ingredients, allergies) = rules;
        // Get a list of possible dictionaries containing the possible ingredient and allergens permutations

        // Check that this rule passes
        if !rules_pass(&rules[1], &allergens) {
            return None;
        }

        let permutations: Vec<HashMap<String, String>> = Vec::new();
        let mut final_result: HashMap<String, String> = HashMap::new();
        for permutation_dict in permutations {
            //match matrix.get([x_val as usize, y_val as usize, z_val as usize]) {
            final_result = match check_recursive(&rules[1..].to_vec(), allergens) {
                Some(mut x) => {
                    return check_recursive(&rules[2..].to_vec(), &mut x)
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
    let input = fs::read_to_string("./src/small.txt").expect("Error reading file");
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
    println!("{:?}", ingredient_allergies[0]);

    // Todo:?? Parse into a vector of rules (closures) lambdas
    // example closure: let line_works = |x as dict| x['mxmxvkd'] == dairy || x['xfxx'] == dairy


    // Always assume that one ingredient can be one allergen only, and vice-versa.
    // (ie: if mxmxvkd contains dairy, no other ingredient can contain dairy)

    // Dynamically build a dictionary of ingredients from each line and try to make it work ?
    let matches = check(&ingredient_allergies);

    // Part 1: Determine which ingredients cannot possibly contain any of the allergens in your list.
    // How many times do any of those ingredients appear?
    let all_ingredients = matches.keys();
    let mut ingredients_without_allergens_sum = 0;
    for (ingredients, _) in ingredient_allergies {
        // get the count of ingredients that are not part of the dict values
        let count_in_line = 0;
        ingredients_without_allergens_sum += count_in_line;
    }


    let sum = 0;
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