use std::cmp::min;

pub fn first_uniq_char(s: String) -> i32 {
    let mut map_occurrences_count = [0_u8; 26];
    let mut map_occurrences_index = [0_usize; 26];

    for (index, character) in s.chars().enumerate() {
        let character_number = (character as u8 - 'a' as u8) as usize;
        map_occurrences_count[character_number] += 1;
        map_occurrences_index[character_number] = index;
    }
    let mut min_unique_char_index = usize::MAX;
    for (letter_index, occurrences_count) in map_occurrences_count.iter().enumerate() {
        if *occurrences_count == 1 {
            min_unique_char_index = min(min_unique_char_index, map_occurrences_index[letter_index]);
        }
    }
    return if min_unique_char_index == usize::MAX { -1 } else { min_unique_char_index as i32};
}
