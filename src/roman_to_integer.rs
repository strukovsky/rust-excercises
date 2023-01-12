use std::collections::HashMap;

pub fn roman_to_int(s: String) -> i32 {
    let mut pairs = HashMap::new();
    pairs.insert("IV", 4);
    pairs.insert("IX", 9);
    pairs.insert("XL", 40);
    pairs.insert("XC", 90);
    pairs.insert("CD", 400);
    pairs.insert("CM", 900);

    let mut singles = HashMap::new();
    singles.insert("I", 1);
    singles.insert("V", 5);
    singles.insert("X", 10);
    singles.insert("L", 50);
    singles.insert("C", 100);
    singles.insert("D", 500);
    singles.insert("M", 1000);

    let length = s.len();
    let mut result = 0;
    let mut previous_was_pair = false;
    for index in 0..length - 1 {
        if previous_was_pair {
            previous_was_pair = false;
            continue;
        }
        if let Some(pair) = pairs.get(&s[index..index + 2]) {
            result += pair;
            previous_was_pair = true;
        } else if let Some(single) = singles.get(&s[index..index + 1]) {
            result += single;
            previous_was_pair = false;
        }
    }
    if !previous_was_pair {
        if let Some(last_digit) = singles.get(&s[length - 1..length]) {
            result += last_digit;
        }
    }
    return result;
}
