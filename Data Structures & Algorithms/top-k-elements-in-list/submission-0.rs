use std::collections::HashMap;

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut cache = HashMap::new();

        // 1. Count frequencies
        for num in nums {
            *cache.entry(num).or_insert(0) += 1;
        }

        // 2. Collect into a Vec to allow sorting
        let mut pairs: Vec<(i32, i32)> = cache.into_iter().collect();

        // 3. Sort by frequency in descending order
        pairs.sort_by(|a, b| b.1.cmp(&a.1));

        // 4. Take the top k keys
        pairs
            .into_iter()
            .take(k as usize)
            .map(|(key, _)| key)
            .collect()
    }
}