use std::collections::HashMap;

fn is_word_meets_criteria(word: &String, mut criteria: HashMap<char, u8>) -> bool {
    for letter in word.chars() {
        if let Some(occurrences_count) = criteria.get_mut(&letter) {
            if *occurrences_count == 0 {
                return false;
            }
            *occurrences_count -= 1;
        } else {
            return false;
        }
    }
    return true;
}

pub fn count_characters(words: Vec<String>, chars: String) -> i32 {
    let mut result = 0;
    let mut mapped_criteria = HashMap::new();
    for char in chars.chars() {
        *mapped_criteria.entry(char).or_insert(0) += 1;
    }
    for word in words.iter() {
        if is_word_meets_criteria(&word, mapped_criteria.clone()) {
            result += word.len();
        }
    }
    return result as i32;
}
