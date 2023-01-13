use std::io::read_to_string;

fn get_closing_parenthesis(opening_parenthesis: &char) -> Option<char> {
    match opening_parenthesis {
        '(' => Some(')'),
        '[' => Some(']'),
        '{' => Some('}'),
        _ => None,
    }
}

pub fn is_valid(s: String) -> bool {
    let opening_parentheses = ['(', '[', '{'];
    let closing_parentheses = [')', ']', '}'];
    let mut stack = vec![];
    for character in s.chars() {
        if opening_parentheses.contains(&character) {
            stack.push(character);
        } else if closing_parentheses.contains(&character) {
            let last_opening_parenthesis = match stack.pop() {
                Some(value) => value,
                None => return false,
            };
            let pairing_parenthesis = match get_closing_parenthesis(&last_opening_parenthesis) {
                Some(value) => value,
                None => return false,
            };
            if pairing_parenthesis != character {
                return false;
            }
        }
    }
    return stack.len() == 0;
}
