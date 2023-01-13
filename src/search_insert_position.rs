use std::cmp::max;

fn __search_insert(nums: &Vec<i32>, scope_start: usize, scope_end: usize, target: i32) -> usize {
    if scope_end - scope_start == 1 {
        if nums[scope_start] < target && target <= nums[scope_end] {
            return scope_end;
        } else if nums[scope_end] < target {
            return scope_end + 1;
        } else if nums[scope_start] > target {
            return if scope_start == 0 { 0 } else { scope_start - 1 };
        }
    }
    if scope_end - scope_start == 0 {
        return if target > nums[scope_start] {
            scope_start + 1
        } else {
            scope_start
        };
    }
    let pivot_index = (scope_start + scope_end) / 2;
    let pivot_element = nums[pivot_index];
    if pivot_element > target {
        __search_insert(nums, scope_start, pivot_index, target)
    } else if pivot_element < target {
        __search_insert(nums, pivot_index, scope_end, target)
    } else {
        return pivot_index;
    }
}

pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    __search_insert(&nums, 0, nums.len() - 1, target) as i32
}
