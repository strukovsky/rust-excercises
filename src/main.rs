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

fn main() {
    println!("{}", search_insert_position::search_insert(vec![1, 3], 3));
}
