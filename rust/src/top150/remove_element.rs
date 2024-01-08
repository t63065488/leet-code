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

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn remove_element_example1() {
        // Given
        let mut nums: Vec<i32> = vec![3, 2, 2, 3];
        let val: i32 = 3;
        // When
        let result = remove_element(&mut nums, val);
        // Then
        assert_eq!(2, result);
        assert_eq!(vec![2, 2], nums);
    }

    #[test]
    fn remove_element_example2() {
        // Given
        let mut nums: Vec<i32> = vec![0, 1, 2, 2, 3, 0, 4, 2];
        let val: i32 = 2;
        // When
        let result = remove_element(&mut nums, val);
        // Then
        assert_eq!(5, result);
        assert_eq!(vec![0, 1, 0, 4, 3], nums);
    }
}
