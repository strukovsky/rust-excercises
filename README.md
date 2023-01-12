# Tasks

## Palindrome linked list

Main concept is to revert given list and then compare reverted and original.  
If palindrome, reverted == original

[Task](https://leetcode.com/problems/palindrome-linked-list)  
[Solution](src/palindrome_linked_list.rs)

## Two sum

Problem is quite simple, no description required

[Task](https://leetcode.com/problems/two-sum)  
[Solution](src/two_sum.rs)


## Binary tree inorder
Main issue was memory management: each component of tree has multiple references on it,
so each TreeNode should be a `Rc<RefCell<T>>` instance.
`Rc` is container for Reference Counted pointer, `RefCell` for dynamic-checked borrowing

**Inorder** (or **infix** order) is when firstly left subtree is taken then root and lasting with right subtree  
[Rc](https://doc.rust-lang.org/std/rc/struct.Rc.html)  
[RefCell](https://doc.rust-lang.org/std/cell/struct.RefCell.html)  
[Task](https://leetcode.com/problems/binary-tree-inorder-traversal)  
[Solution](src/binary_tree_inorder_traversal.rs)  

