#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

use std::rc::Rc;
use std::cell::RefCell;

pub fn __inorder_traversal(root: &Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<i32>) {
    if let Some(node) = root {
        if let node = node.try_borrow().ok() {
            if let Some(node) = node {
                __inorder_traversal(&node.left, result);
                result.push(node.val);
                __inorder_traversal(&node.right, result);
            }
        }
    }
}

pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut result = vec![];
    __inorder_traversal(&root, &mut result);
    return result;
}
