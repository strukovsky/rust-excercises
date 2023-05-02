pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let length = nums.len();
    for (first_index, first_number) in nums.iter().enumerate() {
        for second_index in first_index + 1..length {
            let second_number = nums[second_index];
            if first_number + second_number == target {
                return vec![first_index as i32, second_index as i32];
            }
        }
    }

    return vec![];
}
