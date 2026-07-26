impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut cache: HashMap<i32, usize> = HashMap::new();

        for i in 0..nums.len() {
            if let Some(&index) = cache.get(&nums[i]) {
                return vec![index as i32, i as i32];
            } else {
                cache.insert(target - nums[i], i);
            }
        }

        vec![]
    }
}