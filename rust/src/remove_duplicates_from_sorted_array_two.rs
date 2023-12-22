pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let mut index = 2;
    let mut len = nums.len();
    while index < len {
        if nums[index] == nums[index-1] && nums[index] == nums[index-2] {
            nums.remove(index);
            len -= 1;
        } else {
            index += 1;
        }
    }
    return nums.len() as i32;   
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn remove_duplicates_example1() {
        // Given
        let mut nums: Vec<i32> = vec![1,1,1,2,2,3];
        let expected_nums: Vec<i32> = vec![1,1,2,2,3];
        // When
        let result = remove_duplicates(&mut nums);
        // Then
        assert_eq!(result, nums.len().to_owned() as i32);
        assert_eq!(expected_nums.len(), nums.len());
        assert_eq!(expected_nums, nums);
    }

    #[test]
    fn remove_duplicates_example2() {
        // Given
        let mut nums: Vec<i32> = vec![0,0,1,1,1,1,2,3,3];
        let expected_nums: Vec<i32> = vec![0,0,1,1,2,3,3];
        // When
        let result = remove_duplicates(&mut nums);
        // Then
        assert_eq!(result, nums.len().to_owned() as i32);
        assert_eq!(expected_nums.len(), nums.len());
        assert_eq!(expected_nums, nums);
    }
}