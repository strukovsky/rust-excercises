mod algorithms;
mod structures;
mod patterns;
mod stdlib;

use algorithms::determine_if_string_halves_are_alike::halves_are_alike;

fn main() {

    println!("{}", halves_are_alike(String::from("facwbook")));
}
