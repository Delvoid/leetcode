use std::collections::HashMap;
pub struct Solution;
impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut map: HashMap<char, i64> = HashMap::new();
        s.chars().for_each(|c| {
            *map.entry(c).or_insert(0) += 1;
        });
        for c in t.chars() {
            let count = map.entry(c).or_default();
            *count -= 1;
            if *count == 0 {
                map.remove(&c);
            }
        }

        map.is_empty()
    }

    pub fn is_anagram_v1(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }

        let mut map: HashMap<char, i64> = HashMap::with_capacity(s.len());

        for (a, b) in s.chars().zip(t.chars()) {
            *map.entry(a).or_default() += 1;
            *map.entry(b).or_default() -= 1;
        }

        map.values().all(|&v| v == 0)
    }

    pub fn is_anagram_v2(s: String, t: String) -> bool {
        let mut map: HashMap<char, i64> = HashMap::new();
        s.chars().for_each(|c| {
            *map.entry(c).or_insert(0) += 1;
        });
        t.chars().for_each(|c| {
            *map.entry(c).or_insert(0) -= 1;
        });

        map.values().all(|&v| v == 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_242() {
        let result1 = Solution::is_anagram("anagram".to_string(), "nagaram".to_string());
        assert!(result1, "Test 1 failed. Expected true, got {}", result1);

        let result2 = Solution::is_anagram("rat".to_string(), "car".to_string());
        assert!(!result2, "Test 2 failed. Expected false, got {}", result2);

        let result3 = Solution::is_anagram("a".to_string(), "ab".to_string());
        assert!(!result3, "Test 3 failed. Expected false, got {}", result3);
    }
}
