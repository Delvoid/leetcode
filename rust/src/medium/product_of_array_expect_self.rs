pub struct Solution;
impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut result = vec![1; nums.len()];

        for i in 1..nums.len() {
            result[i] = result[i - 1] * nums[i - 1];
        }

        let mut right = 1;

        for (i, n) in result.iter_mut().enumerate().rev() {
            *n *= right;
            right *= nums[i];
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_238() {
        assert_eq!(
            Solution::product_except_self(vec![1, 2, 3, 4]),
            vec![24, 12, 8, 6]
        );
    }
}

pub struct Solution2;
impl Solution2 {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let nums_len = nums.len();
        let mut vec = vec![0; nums_len];
        let mut prefix = 1;
        for i in 0..nums_len {
            vec[i] = prefix;
            prefix *= nums[i];
        }
        let mut postfix = 1;
        for i in (0..nums_len).rev() {
            vec[i] *= postfix;
            postfix *= nums[i];
        }
        vec
    }
}
