/*
Part 1: Determine the number of messages that completely match rule 0
A pipe (|) means that at least one list of sub-rules must match
 */
use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

fn get_regex_match_dict(rules: &HashMap<usize, &str>) -> HashMap<String, String> {
    // Return a Dict with the regex for possible matches for each rule

    fn get_insert(
        rules: &HashMap<usize, &str>,
        contains: &mut HashMap<String, String>,
        rule: &str,
    ) -> String {
        // Get value and insert if not in dict
        let regex = match contains.get(rule.clone()) {
            Some(x) => x.clone(),
            None => {
                let regex = get_all_possibilities(&rule, rules, contains);
                contains.insert(rule.to_string(), regex.clone());
                regex
            }
        };
        regex
    }

    fn get_all_possibilities(
        rule: &str,
        rules: &HashMap<usize, &str>,
        contains: &mut HashMap<String, String>,
    ) -> String {
        // Get all possibilities for the key (index) and update the contains dict
        if ["\"a\"", "\"b\""].contains(&rule) {
            // Rule is "a" or "b"
            return rule.to_string().replace("\"", "");
        }

        if rule == "42 31 | 42 11 31" {
            // lazy way to handle the repeating patterns for part 2
            // Note this is a balancing pair: (more information here: http://www.regular-expressions.info/balancing.html)
            let (left_rule, right_rule) = ("42", "31");
            let left_regex = get_insert(rules, contains, left_rule);
            let right_regex = get_insert(rules, contains, right_rule);

            // Lazy way to make a regex with count quantifiers that works for Nested Constructs ie. (AB or AABB or AAABBB or AAAABBBB..)
            // It should not be bigger than 5 iterations...
            let mut s = String::new();
            for i in 1..5 {
                let new_string = format!("({}){{{}}}({}){{{}}}", left_regex, i, right_regex, i);
                if i == 1 {
                    s.push_str(new_string.as_str());
                } else {
                    s.push_str(format!("|{}", new_string).as_str());
                }
            }
            return format!("({})", s);
        }

        if rule == "42 | 42 8" {
            // lazy way to handle the repeating patterns for part 2
            let left_rule = "42";
            let left_regex = get_insert(rules, contains, &left_rule);
            // It's a repeating pattern
            return format!("({}+)", left_regex);
        }

        if rule.contains("|") {
            let (left_rule, right_rule) = rule
                .splitn(2, " | ")
                .collect_tuple()
                .expect("Error getting tuple");

            let left_regex = get_insert(rules, contains, left_rule);
            let right_regex = get_insert(rules, contains, right_rule);
            return format!("({}|{})", left_regex, right_regex);
        } else if rule.contains(" ") {
            let (left_rule, right_rule) = rule
                .splitn(2, char::is_whitespace)
                .collect_tuple()
                .expect("Error getting tuple");

            let left_regex = get_insert(rules, contains, left_rule);
            let right_regex = get_insert(rules, contains, right_rule);

            return format!("{}{}", left_regex, right_regex);
        } else {
            // Parse rule to get the index
            let rule_index: usize = rule.parse().expect("Usize parsing issue");
            let rule = rules.get(&rule_index).expect("index error");
            return get_insert(rules, contains, rule);
        }
    }

    let mut contains: HashMap<String, String> = HashMap::new();
    for rule in rules.values() {
        let re = get_all_possibilities(rule, rules, &mut contains);
        contains.insert(rule.to_string(), re);
    }
    contains
}

fn get_sum_for_rule(input: Vec<&str>, re_str: &String) -> usize {
    // Return the sum of all inputs matching the regex rule
    let regex_for_rule = format!("^{}$", re_str);
    let re = Regex::new(regex_for_rule.as_str()).expect("Invalid regex");
    input.iter().filter(|x| re.is_match(x)).count()
}

fn main() {
    let file_path = PathBuf::from("./src/input.txt");
    let f = fs::read_to_string(&file_path).expect("Error reading file");

    // Need to split on all empty lines
    let input_entries: Vec<_> = Regex::new(r"\n\n").unwrap().split(&f).collect::<Vec<_>>();

    // Create a hashmap of rules
    let mut rules: HashMap<usize, &str> = HashMap::new();
    for line in input_entries[0].lines() {
        let (rule_num, rule_str) = line
            .splitn(2, ": ")
            .collect_tuple()
            .expect("Error getting tuple");
        let rule_num: usize = rule_num.parse().expect("Error parsing");
        rules.insert(rule_num, rule_str);
    }

    // Build a dict with the index and possible combinations
    let contains: HashMap<String, String> = get_regex_match_dict(&rules);

    // Check all the messages against the selected rule index
    let rule = rules.get(&0).expect("Error");
    let rule_regex = contains.get(*rule).expect("Error");
    let sum_match = get_sum_for_rule(input_entries[1].lines().collect_vec(), rule_regex);
    println!(
        "The message count that match rule 0 for part 1 is {}",
        sum_match
    );

    // Part 2; update the rule 8 and 11.
    let mut new_rules = rules.clone();
    let rule_8 = new_rules.entry(8).or_insert("");
    *rule_8 = "42 | 42 8";
    let rule_11 = new_rules.entry(11).or_insert("");
    *rule_11 = "42 31 | 42 11 31";

    // Build a dict with the index and possible combinations
    let contains: HashMap<String, String> = get_regex_match_dict(&new_rules);

    // Check all the messages against the selected rule index
    let rule_regex = contains.get(*rule).expect("Error");
    let sum_match = get_sum_for_rule(input_entries[1].lines().collect_vec(), rule_regex);

    println!(
        "The message count that match rule 0 for part 2 is {}",
        sum_match
    );
}
