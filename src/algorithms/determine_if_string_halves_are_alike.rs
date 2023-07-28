pub fn halves_are_alike(s: String) -> bool {
    let lowercased = s.to_ascii_lowercase();
    let vowels_string = String::from("aeiou");
    let vowels = vowels_string.as_bytes();
    let string = lowercased.as_bytes();
    let length = string.len();

    let mut front_counter = 0;
    let mut back_counter = 0;
    for i in 0..length {
        let reverse_i = length - 1 - i;
        if i == reverse_i + 1 {
            break;
        }
        let front_char = string[i];
        let back_char = string[reverse_i];
        if vowels.binary_search(&front_char).is_ok() {
            front_counter += 1;
        }
        if vowels.binary_search(&back_char).is_ok() {
            back_counter += 1;
        }
    }
    front_counter == back_counter
}
