pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) -> Vec<i32> {
    nums1.truncate(m as usize);
    nums2.truncate(n as usize);
    nums1.append(nums2);
    nums1.sort_unstable();
    return nums1.to_owned();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_example1() {
        // Given
        let mut vec_a = vec![1, 2, 3, 0, 0, 0];
        let mut vec_b = vec![2, 5, 6];
        // When
        let result = merge(&mut vec_a, 3, &mut vec_b, 3);
        // Then
        assert_eq!(6, result.len());
        assert_eq!(vec![1, 2, 2, 3, 5, 6], result);
    }

    #[test]
    fn test_merge_example2() {
        // Given
        let mut vec_a = vec![1];
        let mut vec_b = vec![];
        // When
        let result = merge(&mut vec_a, 1, &mut vec_b, 0);
        // Then
        assert_eq!(1, result.len());
        assert_eq!(vec![1], result);
    }

    #[test]
    fn test_merge_example3() {
        // Given
        let mut vec_a = vec![0];
        let mut vec_b = vec![1];
        // When
        let result = merge(&mut vec_a, 0, &mut vec_b, 1);
        // Then
        assert_eq!(1, result.len());
        assert_eq!(vec![1], result);
    }
}
