use std::collections::HashSet;

pub fn get_squared_sum(number: i32) -> i32 {
    let mut iter_number = number;
    let mut result = 0;
    while iter_number > 0 {
        result += (iter_number % 10).pow(2);
        iter_number /= 10;
    }
    result
}

pub fn is_happy(n: i32) -> bool {
    let mut iter_number = n;
    let mut met_values: HashSet<i32> = HashSet::new();
    met_values.insert(n);
    while iter_number != 1 {
        let value = get_squared_sum(iter_number);
        if !met_values.contains(&value) {
            met_values.insert(value);
            iter_number = value;
        } else {
            return false
        }
    }
    return true;
}
