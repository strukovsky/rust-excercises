use std::cell::RefCell;
use std::rc::Rc;

mod palindrome_linked_list;
mod two_sum;
mod binary_tree_inorder_traversal;

fn main() {
    let one = Some(Rc::new(RefCell::new(binary_tree_inorder_traversal::TreeNode::new(1))));
    let three = Some(Rc::new(RefCell::new(binary_tree_inorder_traversal::TreeNode::new(3))));
    let two = binary_tree_inorder_traversal::TreeNode {
        left: one,
        right: three,
        val: 2,
    };
    let result = binary_tree_inorder_traversal::inorder_traversal(Some(Rc::new(RefCell::new(two))));
    println!("{} {} {}", result[0], result[1], result[2]);
}
