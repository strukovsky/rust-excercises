use std::cell::RefCell;
use std::rc::Rc;


use crate::binary_tree_inorder_traversal::TreeNode;


pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    fn helper(nums: &Vec<i32>, start_scope: usize, end_scope: usize) -> Option<Rc<RefCell<TreeNode>>> {
        if end_scope - start_scope == 0 {
            return None;
        }
        let pivot_index = (end_scope + start_scope) / 2;
        let pivot_element = nums[pivot_index];
        let mut root = TreeNode::new(pivot_element);
        root.left = helper(nums, start_scope, pivot_index);
        root.right = helper(nums, pivot_index + 1, end_scope);
        return Some(Rc::new(RefCell::new(root)));
    }
    helper(&nums, 0, nums.len())
}
