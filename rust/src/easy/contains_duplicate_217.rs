use std::collections::HashSet;
pub struct Solution;

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut map = HashSet::with_capacity(nums.len());
        map.insert(nums[0]);

        for num in &nums[1..] {
            if map.contains(num) {
                return true;
            }

            map.insert(*num);
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_217() {
        let result1 = Solution::contains_duplicate(vec![1, 2, 3, 1]);
        assert!(result1, "Test 1 failed. Expected true, got {}", result1);

        let result2 = Solution::contains_duplicate(vec![1, 2, 3, 4]);
        assert!(!result2, "Test 2 failed. Expected True, got {}", result2);

        let result3 = Solution::contains_duplicate(vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2]);
        assert!(result3, "Test 3 failed. Expected true, got {}", result3);
    }
}
