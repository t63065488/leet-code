pub fn rotate(nums: &mut Vec<i32>, k: i32) {
    nums.rotate_right(k as usize);
}

pub fn rotate_slice(nums: &mut Vec<i32>, k: i32) {
    let mut new_nums: Vec<i32> = Vec::new();
    new_nums.extend_from_slice(&nums[(nums.len() - k as usize)..nums.len()]);
    new_nums.extend_from_slice(&nums[0..(nums.len() - k as usize)]);
    nums.clear();                                                
    nums.append(&mut new_nums);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_rotate_example1() {
        // Given
        let mut nums: Vec<i32> = vec![1,2,3,4,5,6,7];
        let k: i32 = 3;
        let expected_result: Vec<i32> = vec![5,6,7,1,2,3,4];
        // When
        rotate_slice(&mut nums, k);
        // Then
        assert_eq!(nums, expected_result);
    }
    
    #[test]
    pub fn test_rotate_example2() {
        // Given
        let mut nums: Vec<i32> = vec![-1,-100,3,99];
        let k: i32 = 2;
        let expected_result: Vec<i32> = vec![3, 99, -1, -100];
        // When
        rotate_slice(&mut nums, k);
        // Then
        assert_eq!(nums, expected_result);
    }
}