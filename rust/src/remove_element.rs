pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    let mut indices: Vec<usize> = Vec::new();
    for (pos, e) in nums.iter().enumerate() {
        if e.to_owned() == val {
            indices.push(pos);
        }
    }
    indices.reverse();
    for index in indices {
        nums.swap_remove(index);
    }
    return nums.len() as i32;
}