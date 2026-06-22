use std::collections::HashMap;

use crate::common::Solution;

impl Solution {
    pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
        fn dfs(
            i: usize,
            sum: i32,
            memo: &mut HashMap<(usize, i32), i32>,
            nums: &[i32],
            target: i32,
        ) -> i32 {
            if i == nums.len() {
                return if sum == target { 1 } else { 0 };
            }

            if let Some(&val) = memo.get(&(i, sum)) {
                return val;
            }

            let res = dfs(i + 1, sum + nums[i], memo, nums, target)
                + dfs(i + 1, sum - nums[i], memo, nums, target);
            memo.insert((i, sum), res);
            res
        }
        let mut memo = HashMap::new();
        dfs(0, 0, &mut memo, &nums, target)
    }
}
