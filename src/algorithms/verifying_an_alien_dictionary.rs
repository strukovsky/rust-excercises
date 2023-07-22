use std::collections::HashMap;

pub fn first_earlier_than_second(first: &String, second: &String, dictionary: &HashMap<char, usize>) -> bool {
    if first.len() < second.len() {
        return !first_earlier_than_second(second, first, dictionary);
    }
    for (index, first_character) in first.chars().enumerate() {
        if let Some(second_character) = second.chars().nth(index) {
            let first_dictionary_index = *dictionary.get(&first_character).unwrap() as i32;
            let second_dictionary_index = *dictionary.get(&second_character).unwrap() as i32;
            let diff = first_dictionary_index - second_dictionary_index;
            if diff == 0 {
                continue;
            }
            if diff > 0 {
                return false;
            }
            if diff < 0 {
                return true;
            }
        } else { return false; };
    }
    return true;
}

pub fn is_alien_sorted(words: Vec<String>, order: String) -> bool {
    let mut dictionary = HashMap::new();
    for (index, character) in order.chars().enumerate() {
        dictionary.insert(character, index);
    }
    let words_count = words.len();
    for i in 0..words_count - 1 {
        if let Some(first) = words.iter().nth(i) {
            if let Some(second) = words.iter().nth(i + 1) {
                if !first_earlier_than_second(&first, &second, &dictionary) {
                    return false;
                }
            }
        }
    }
    true
}
