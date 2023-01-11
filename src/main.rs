
mod palindrome_linked_list;

fn main() {
    let list = palindrome_linked_list::ListNode {
        val: 1,
        next: Option::from(Box::new(palindrome_linked_list::ListNode {
            val: 2,
            next: Option::from(Box::new(palindrome_linked_list::ListNode {
                val: 1,
                next: None,
            })),
        },))
    };


    println!("{}", palindrome_linked_list::is_palindrome(Option::from(Box::new(list))));
}
