pub fn find_the_difference(s: String, t: String) -> char {
    let mut s_sum = 0;
    s.chars().for_each(|ch| {s_sum += ch as i32});
    let mut t_sum = 0;
    t.chars().for_each(|ch| {t_sum += ch as i32});
    return ((t_sum - s_sum) as u8) as char;
}
