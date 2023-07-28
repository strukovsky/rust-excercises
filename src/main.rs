mod algorithms;
mod structures;
mod patterns;
mod stdlib;

use stdlib::rc_refcell::Tree;

fn main() {
    let root = Tree::new(10);
    root.push(1);
    root.push(2);
    root.push(3);
    root.push(11);
    root.push(12);
    root.push(13);
    root.traverse(|data: i32| {println!("{data:?}")});
}
