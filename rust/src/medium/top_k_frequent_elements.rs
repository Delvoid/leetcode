use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut map = HashMap::new();
        for num in nums {
            let count = map.entry(num).or_insert(0);
            *count += 1;
        }
        let mut pairs: Vec<(i32, i32)> = map.into_iter().collect();
        pairs.sort_by(|a, b| b.1.cmp(&a.1));
        pairs.iter().take(k as usize).map(|(num, _)| *num).collect()
    }
}
