use std::collections::HashMap;
pub struct Solution;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map: HashMap<String, Vec<String>> = HashMap::new();

        for s in strs {
            let mut chars = s.chars().collect::<Vec<char>>();
            chars.sort();
            let sorted_s = chars.into_iter().collect::<String>();
            map.entry(sorted_s).or_insert(Vec::new()).push(s);
        }

        map.into_values().collect()
    }
}

pub struct Solution2;

impl Solution2 {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map: HashMap<[u8; 26], Vec<String>> = HashMap::new();

        for s in strs {
            let mut key = [0; 26];
            for c in s.chars() {
                key[(c as u8 - b'a') as usize] += 1;
            }

            map.entry(key).or_insert(Vec::new()).push(s);
        }

        map.into_values().collect()
    }
}
