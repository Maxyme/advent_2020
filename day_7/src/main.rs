/*How many bag colors can eventually contain at least one of the selected bag?
Find recursively the number of outer bags that can contain the selected bag following the input rules */

use itertools::Itertools;
use regex::Regex;
use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;

fn get_bag_colours_can_contain(selected_color: &str, all_entries: &Vec<&str>) -> Vec<String> {
    // Check whether a bag can contain the selected values
    let mut return_colors = Vec::<String>::new();
    for entry in all_entries {
        let (container, can_contain) = entry.splitn(2, "contain").collect_tuple().unwrap();
        if can_contain.contains(&selected_color) {
            let bag_color = container.replace(" bags", "");
            return_colors.push(bag_color)
        }
    }
    return_colors
}

fn get_bags_can_contain_recursively(
    selected_color: &str,
    all_entries: &Vec<&str>,
    bags: &mut HashSet<String>,
) {
    let bags_can_countain = get_bag_colours_can_contain(selected_color, &all_entries);
    for bag_colour in bags_can_countain {
        &bags.insert(bag_colour.to_string());
        get_bags_can_contain_recursively(&*bag_colour, &all_entries, bags);
    }
    return;
}

fn get_bag_content(selected_color: &str, all_entries: &Vec<&str>) -> Vec<(String, i32)> {
    // Find the line that start with the selected color and return a vec of
    // tuples containing the colors and count of bags it contain
    let line: &str = all_entries
        .iter()
        .filter(|x| x.starts_with(selected_color))
        .next()
        .unwrap();
    let re = Regex::new(r"([0-9])(.*?)bags?").unwrap();

    let result_matches: Vec<&str> = re.find_iter(line).map(|x| x.as_str()).collect();
    let mut results = Vec::<(String, i32)>::new();
    for result in result_matches {
        let (count, color) = result
            .splitn(2, char::is_whitespace)
            .collect_tuple()
            .unwrap();
        let count = count.parse::<i32>().expect("Expect int");
        let (color, _) = color.splitn(2, "bag").collect_tuple().unwrap();
        results.push((color.to_string(), count));
    }
    results
}

fn get_total_num_bags_color_can_contain(selected_color: &str, all_entries: &Vec<&str>) -> i32 {
    let bag_contains = get_bag_content(selected_color, &all_entries);
    let mut total_count = 0;
    for (color, count) in bag_contains {
        let count_for_color = get_total_num_bags_color_can_contain(&color, &all_entries);
        total_count += count + count * count_for_color; // must add first count as well
    }
    total_count
}

fn main() {
    let file_path = PathBuf::from("./src/input.txt");
    let f = fs::read_to_string(&file_path).expect("Error reading file");
    let input_entries: Vec<_> = f.lines().collect::<Vec<_>>(); // split on empty lines
    let selected_color = "shiny gold bag";

    // part 1: Get the total number of bags that can contain the selected color
    let mut bags: HashSet<String> = HashSet::new();
    get_bags_can_contain_recursively(selected_color, &input_entries, &mut bags);
    println!("The total number of bags is {}", bags.len());

    // part 2: Get the number of bags the selected colour must contain
    let total = get_total_num_bags_color_can_contain(selected_color, &input_entries);
    println!(
        "The total number of bags {} color must contain is {}",
        selected_color, total
    );
}
