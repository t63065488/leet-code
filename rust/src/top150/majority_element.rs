use std::collections::HashMap;

pub fn majority_element(nums: Vec<i32>) -> i32 {
    let mut map: HashMap<i32, i32> = HashMap::new();
    let default_value: i32 = 0;
    let n = (nums.len() / 2) as i32;
    for num in nums {
        map.insert(num, *map.get(&num).unwrap_or(&default_value) + 1);
    }
    for (key, value) in map {
        if value > n {
            return key;
        }
    }
    return 0;        
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_majority_element_example1() {
        // Given
        let nums: Vec<i32> = vec![3,2,3];
        let expected_outcome: i32 = 3;
        // When
        let result: i32 = majority_element(nums);
        // Then
        assert_eq!(result, expected_outcome);
    }

    
    #[test]
    fn test_majority_element_example2() {
        // Given
        let nums: Vec<i32> = vec![2,2,1,1,1,2,2];
        let expected_outcome: i32 = 2;
        // When
        let result: i32 = majority_element(nums);
        // Then
        assert_eq!(result, expected_outcome);
    }
}

