mod algorithms;
mod structures;
mod patterns;
mod stdlib;

use structures::example_box::ExampleBox;

fn main() {
    let boxed = ExampleBox::new(1);
    let deref = *boxed;
    println!("{deref:?}");
}
