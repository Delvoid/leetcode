use std::collections::HashMap;
pub struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: HashMap<i32, i64> = HashMap::with_capacity(nums.len());

        for (i, &num) in nums.iter().enumerate() {
            let diff = target - num;
            if map.contains_key(&diff) {
                return vec![map[&diff] as i32, i as i32];
            }
            map.insert(num, i as i64);
        }
        vec![]
    }
}
