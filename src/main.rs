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


fn main() {
    let nums = vec![1, 2, 3, 4, 5, 6];
    let tree = convert_sorted_array_to_binary_search_tree::sorted_array_to_bst(nums);
    let a = binary_tree_inorder_traversal::inorder_traversal(tree);
    let q = vec![1, 2, 3, 4, 5, 6];
    println!("{}", q == a);
}
