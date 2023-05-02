use std::collections::HashMap;

pub fn majority_element(nums: Vec<i32>) -> i32 {
    let occurrences_to_be_majority = nums.len() / 2;
    let mut occurrences = HashMap::new();
    for num in nums.iter() {
        *occurrences.entry(*num).or_insert(0) += 1;
    }
    for (key, value) in occurrences.iter() {
        if *value > occurrences_to_be_majority {
            return *key as i32;
        }
    }
    return 0;
}
