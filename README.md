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

## Palindrome number
Palindrome number is when iterating a number from end and from start we have the same digits
Main difficulty is to iterate number in two directions without string or vector usage.
To iterate from bigger to lower digits,

1. take digit from current position: `digit = number / 10 ** (digit_count - 1)`
2. go to next iteration `number -= digit * 10 ** digit_count`

To iterate from lower to upper digits,

1. take digit: `digit = number % 10`
2. go to next iteration `number /= 10`

[Task](https://leetcode.com/problems/palindrome-number)
[Solution](src/palindrome_number.rs)

## Roman to integer
Quite easy task, no explanations required

[Task](https://leetcode.com/problems/roman-to-integer)  
[Solution](src/roman_to_integer.rs)
