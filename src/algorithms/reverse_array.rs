use std::mem::{take, replace};

pub fn reverse_array<T: Default>(array: &mut Vec<T>) {
    let mut reversed: Vec<T> = Vec::new();
    for item in array.iter_mut().rev() {
        reversed.push(take(item));
    }
    let previous_result = replace(array, reversed);
}
