pub fn is_palindrome(x: i32) -> bool {
    if x < 0 {
        return false;
    }
    let mut digit_count = 0_u32;
    let mut iterated_number = x;
    while iterated_number > 0 {
        iterated_number /= 10;
        digit_count += 1;
    }
    iterated_number = x;
    let mut iterated_backwards_number = x;
    while iterated_backwards_number > 0{
        let digit_from_end = iterated_backwards_number % 10;
        let digit_from_start = iterated_number / 10_i32.pow(digit_count - 1);
        println!("{} {}", digit_from_start, digit_from_end);
        digit_count -= 1;
        iterated_backwards_number /= 10;
        iterated_number -= digit_from_start * 10_i32.pow(digit_count);
        if digit_from_end != digit_from_start {
            return false;
        }
    }
    return true;
}
