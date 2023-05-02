mod algorithms;
mod structures;
fn main() {
   let mut dequeue  = structures::dequeue::Dequeue::new();
    dequeue.push_front(1);
    dequeue.push_front(2);
    dequeue.push_front(3);
    let item = dequeue.get_from_front(1);
    let mut item_borrowed = item.borrow_mut();
    println!("Before {}", item_borrowed);
    *item_borrowed = 100;
    println!("After {}", item_borrowed);
}
