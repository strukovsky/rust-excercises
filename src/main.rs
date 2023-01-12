use std::cell::RefCell;
use std::rc::Rc;

mod palindrome_linked_list;
mod two_sum;
mod binary_tree_inorder_traversal;
mod palindrome_number;
mod roman_to_integer;
mod longest_common_prefix;

fn main() {
    let mut vec = Vec::new();
    vec.push(String::from("reflower"));
    vec.push(String::from("flow"));
    vec.push(String::from("flight"));
    println!("{}", longest_common_prefix::longest_common_prefix(vec));
}
