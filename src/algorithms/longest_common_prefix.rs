use std::collections::HashMap;

pub fn longest_common_prefix(strs: Vec<String>) -> String {
    let mut prefixes = HashMap::new();
    let strs_len = strs.len();
    let first_string = &strs[0];
    for index in 0..first_string.len() {
        let prefix = &first_string[0..index + 1];
        prefixes.insert(String::from(prefix), 1);
    }

    for string in &strs[1..] {
        for (prefix, count) in prefixes.iter_mut() {
            if string.as_str().starts_with(prefix.as_str()) {
                *count += 1;
            }
        }
    }

    let mut max_prefix = String::from("");
    for (prefix, count) in prefixes.iter() {
        if *count == strs_len {
            if prefix.len() > max_prefix.len() {
                max_prefix = prefix.clone();
            }
        }
    }

    return String::from(max_prefix);
}
