impl Solution {
    pub fn has_duplicate(nums: Vec<i32>) -> bool {
        let mut x = 0;
        for i in &nums{
            x = x ^ i;
        }
        x != nums.len().try_into().unwrap()
    }
}
