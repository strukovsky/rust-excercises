pub fn pascal_triangle(num_rows: u32) -> Vec<Vec<i32>> {
    let mut result = vec![vec![1]];
    for i in 1..num_rows {
        let mut current_row: Vec<i32> = vec![];
        let previous_row = &result[(i - 1) as usize];
        let current_row_length = i + 1;
        let current_inner_elements = current_row_length - 2;
        current_row.push(1);
        for current_inner_element in 0..current_inner_elements {
            current_row.push(previous_row[current_inner_element as usize] + previous_row[(current_inner_element + 1) as usize])
        }
        current_row.push(1);
        result.push(current_row);
    }
    result
}
