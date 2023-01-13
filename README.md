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

## Longest common prefix

This task bases on hashtable usage. As soon as we have to find a common
prefix we can easily take random string from string pool and split it into hashtable
I prefer take the first string from pool.
Example: string `test`
Hashtable will be:

| key  | value |
|------|-------|
| t    | 1     |
| te   | 1     |
| tes  | 1     |
| test | 1     |

We place 1 instead of 0 because at least in one string this prefix is presented (in the first in my example or in random)
Then we have to iterate other the pool matching whether prefix is the beginning of each string
If some string in the pool matches we increase in hashtable corresponding entry's value

Example: if pool is `[test, tears, technical]` then hashtable after all iterations will be

| key  | value |
|------|-------|
| t    | 3     |
| te   | 3     |
| tes  | 1     |
| test | 1     |

And the last step is to find in hashtable an entry which has as large value as is size
of the pool and then take entry with longest key.  
This longest key will be longest common prefix

[Task](https://leetcode.com/problems/longest-common-prefix)  
[Solution](src/longest_common_prefix.rs)  

## Valid parentheses

The idea is quite simple: use `Vec<char>` as a stack where we store opening parentheses.   
Iterating through the string we match whether symbol is opening or closing parenthesis

If it is opening parenthesis add it to stack
If it is closing parenthesis:
1) pop from stack last opening parenthesis
2) if stack is empty `return false`
3) find a closing match for this opening parenthesis (like `'('` goes to `')'`, etc)
4) if closing parenthesis for popped open one isn't the same as current character `return false`
When all string is done, return the fact whether stack is empty 
(obviously it should be empty if string is valid: every opening parenthesis was matched with closing)

[Task](https://leetcode.com/problems/valid-parentheses)  
[Solution](src/valid_parentheses.rs)


## Search insert position
Task requires simple recursion and a lot of is-else branches

[Task](https://leetcode.com/problems/search-insert-position)  
[Solution](src/search_insert_position.rs)  

## Convert sorted array to binary search tree
Task is solved with recursion and is completely like previous task

[Task](https://leetcode.com/problems/convert-sorted-array-to-binary-search-tree)  
[Solution](src/convert_sorted_array_to_binary_search_tree.rs)  
