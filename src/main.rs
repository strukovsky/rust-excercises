mod palindrome_linked_list;
mod two_sum;

fn main() {
    let result = two_sum::two_sum(vec![1, 2, 3, 4, 5], 5);
    println!("{} {}", result[0], result[1]);
}
