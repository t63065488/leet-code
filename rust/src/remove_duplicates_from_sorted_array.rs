pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let mut unique_entries: Vec<i32> = Vec::new();
    for num in &mut *nums {
        if !unique_entries.contains(num) {
            unique_entries.push(*num);
        }
    }
    nums.clear();
    nums.append(&mut unique_entries);
    return nums.len().to_owned() as i32;
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn remove_duplicates_example1() {
        // Given
        let mut nums: Vec<i32> = vec![1,1,2];
        let expected_nums: Vec<i32> = vec![1,2];
        let expected_result: i32 = expected_nums.len().to_owned() as i32;
        // When
        let result = remove_duplicates(&mut nums);
        // Then
        assert_eq!(expected_result, result);
        assert_eq!(expected_nums, nums);
    }

    
    #[test]
    fn remove_duplicates_example2() {
        // Given
        let mut nums: Vec<i32> = vec![0,0,1,1,1,2,2,3,3,4];
        let expected_nums: Vec<i32> = vec![0,1,2,3,4];
        let expected_result: i32 = expected_nums.len().to_owned() as i32;
        // When
        let result = remove_duplicates(&mut nums);
        // Then
        assert_eq!(expected_result, result);
        assert_eq!(expected_nums, nums);
    }
}