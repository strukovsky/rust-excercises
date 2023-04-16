use std::cell::RefCell;
use std::rc::Rc;

mod palindrome_linked_list;
mod two_sum;
mod binary_tree_inorder_traversal;
mod palindrome_number;
mod roman_to_integer;
mod longest_common_prefix;
mod valid_parentheses;
mod search_insert_position;
mod convert_sorted_array_to_binary_search_tree;
mod first_unique_character_in_a_string;
mod majority_element;
mod find_words_that_can_be_formed_by_characters;
mod pascal_triangle;
mod find_the_difference;
mod is_happy;
mod container;


fn main() {
    let mut container: container::Container<i32> = container::Container::new();
    container.push(10);
    container.push(10);
    container.push(10);
    container.push(20);
    println!("{}", container.get(0).unwrap());
    println!("Size {} Unique {}", container.get_size(), container.get_unique_items_count());
    for item in container.iter() {
        println!("{}", *item);
    }
}
