/*
The rules of operator precedence have changed. Rather than evaluating multiplication before addition,
the operators have the same precedence, and are evaluated left-to-right regardless of the order in which they appear.

ie 2 * 3 + (4 * 5) -> 26

Evaluate the expression on each line of the homework; what is the sum of the resulting values.

 */
use std::fs;
use std::path::PathBuf;

fn get_last_operator(last_stack: &Vec<(usize, bool, char)>, add_before_mul: bool) -> (usize, bool) {
    // Return the last operator depending on priority
    let last_value = last_stack.last().expect("Expected tuple");
    let mut last_index = last_value.0;
    let mut last_operator_in_parenthesis = last_value.1;
    if add_before_mul {
        // Return the last one that is a multiplication, ie: 1x4+5 -> index 1 instead of 3
        for (last_operator_index, inside_parenthesis, operator) in last_stack.iter().rev() {
            if operator == &'*' {
                last_index = *last_operator_index;
                last_operator_in_parenthesis = *inside_parenthesis;
                break;
            }
        }
    }
    return (last_index, last_operator_in_parenthesis);
}

fn get_expression_parts(expression: &str, add_before_mul: bool) -> (String, String, char) {
    // Split on parenthesis first, then on operators to get a left and right part
    let expression: String = expression.chars().filter(|c| !c.is_whitespace()).collect();

    let mut parenthesis_stack: Vec<usize> = Vec::new();
    let mut operator_stack: Vec<Vec<(usize, bool, char)>> = Vec::new();

    for (index, char) in expression.chars().enumerate() {
        if ['+', '*'].contains(&char) {
            // Add the operator index to the stack , and create a new vec if none exists
            if operator_stack.len() == 0 {
                operator_stack.push(Vec::new());
            }
            let last_in_stack = operator_stack.len() - 1;
            if let Some(elem) = operator_stack.get_mut(last_in_stack) {
                let operator_inside_parenthesis = parenthesis_stack.len() > 0;
                elem.push((index, operator_inside_parenthesis, char));
            }
        } else if char == '(' {
            parenthesis_stack.push(index);
            operator_stack.push(Vec::new());
        } else if char == ')' {
            // Only pop if not at the end of the expression or if the stack is bigger than 1
            if operator_stack.len() > 1 || index < expression.len() - 1 {
                operator_stack.pop();
            }
            parenthesis_stack.pop();
        }
    }

    // get the last operator from the stack, or the last multiplication if prioritizing addition order
    let last_stack: &Vec<(usize, bool, char)> =
        operator_stack.last().expect("Error retrieving last value");
    let (last_operator_index, inside_parenthesis) = get_last_operator(&last_stack, add_before_mul);

    let mut left_part = String::from(&expression[..last_operator_index]);
    let mut right_part = String::from(&expression[last_operator_index + 1..]);

    if inside_parenthesis {
        // Quick fix for parenthesis that would be left open
        left_part = String::from(&left_part[1..]);
        right_part = String::from(&right_part[0..right_part.len() - 1]);
    }

    let operator: char = expression
        .chars()
        .nth(last_operator_index)
        .expect("Error indexing string");
    (left_part, right_part, operator)
}

fn operation(left: u64, right: u64, operator: char) -> u64 {
    // Return the result of left and right sides
    match operator {
        '+' => left + right,
        '*' => left * right,
        _ => panic!("Unexpected"),
    }
}

fn resolve(expression: &str, add_before_mul: bool) -> u64 {
    // parse an equation in left and right sides to resolve it recursively
    let is_number = expression.chars().all(|x| x.is_numeric());
    if is_number {
        // Return number if it contains only numbers
        return expression
            .parse::<u64>()
            .expect("Error parsing str into u32");
    }
    let (left, right, operator) = get_expression_parts(expression, add_before_mul);
    return operation(
        resolve(&*left.to_string(), add_before_mul),
        resolve(&*right, add_before_mul),
        operator,
    );
}

fn main() {
    let file_path = PathBuf::from("./src/input.txt");
    let f = fs::read_to_string(&file_path).expect("Error reading file");
    let lines: Vec<&str> = f.lines().collect();

    let mut sum = 0;
    for line in &lines {
        sum += resolve(line, false) as u64;
    }

    println!("The sum of all expressions for part 1 is {}", sum);

    let mut sum = 0;
    for line in &lines {
        sum += resolve(line, true) as u64;
    }

    println!("The sum of all expressions for part 2 is {}", sum);
}

#[cfg(test)]
mod tests {
    // Import names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_resolve_no_order() {
        let input = "(2 + 3)";
        assert_eq!(resolve(input, false), 5);

        // double parenthesis
        let input = "(7 * 3 + (4 + 5 + 6))";
        assert_eq!(resolve(input, false), 36);

        let input = "((2 + 3) + (4 + 5))";
        assert_eq!(resolve(input, false), 14);

        let input = "2 * 3 + (4 * 5)";
        assert_eq!(resolve(input, false), 26);

        let input = "(2 + 6) * 2 + 2 + 4";
        assert_eq!(resolve(input, false), 22);

        let input = "5 + (8 * 3 + 9 + 3 * 4 * 3)";
        assert_eq!(resolve(input, false), 437);

        let input = "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))";
        assert_eq!(resolve(input, false), 12240);

        let input = "7 * ((7 + 4) + 7 * (3 + 4 * 8 + 7 * 8) * 2 * (4 + 3 + 9 * 5)) + (9 * 6) + 8";
        assert_eq!(resolve(input, false), 10160702);
    }

    #[test]
    fn test_resolve_add_before_mult() {
        let input = "2 * 3 + (4 * 5)";
        assert_eq!(resolve(input, true), 46);

        let input = "(1*1)+1";
        assert_eq!(resolve(input, true), 2);

        let input = "1+(1*1)";
        assert_eq!(resolve(input, true), 2);

        let input = "((7+4)+7*(3+4*8+7*8)*2*(4+3+9*5))+(9*6)+8";
        assert_eq!(resolve(input, true), 2419262);
    }
}
