use std::cell::RefCell;
use std::rc::Rc;

mod palindrome_linked_list;
mod two_sum;
mod binary_tree_inorder_traversal;
mod palindrome_number;

fn main() {
    println!("{}", palindrome_number::is_palindrome(121));
}
