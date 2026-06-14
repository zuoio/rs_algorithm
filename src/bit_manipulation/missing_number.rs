use crate::common::Solution;

impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let n = nums.len() as i32;
        let mut xorr = n;
        for i in 0..nums.len() {
            xorr ^= i as i32 ^ nums[i];
        }
        xorr
    }
}
