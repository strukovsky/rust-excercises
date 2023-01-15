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


fn main() {
    let result = find_words_that_can_be_formed_by_characters::count_characters(
        vec![String::from("baba"), String::from("cac")],
        String::from("aabbcc")
    );
    println!("{}", result);
}
