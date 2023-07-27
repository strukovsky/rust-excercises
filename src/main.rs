mod algorithms;
mod structures;
mod patterns;

use algorithms::reverse_array::reverse_array;

fn main() {
   let mut a = vec![1, 2, 3, 4, 5];
   reverse_array(&mut a);
   println!("{a:?}");
}
