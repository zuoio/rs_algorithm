use crate::common::Solution;

impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        let total: i32 = nums.iter().sum();
        if total % 2 != 0 {
            return false;
        }
        let target = total as usize / 2;
        let n = nums.len();
        let mut dp = vec![vec![false; target + 1]; n + 1];

        for i in 0..=n {
            dp[i][0] = true;
        }

        for i in 1..=n {
            for j in 1..=target {
                if nums[i - 1] as usize <= j {
                    dp[i][j] = dp[i - 1][j] || dp[i - 1][j - nums[i - 1] as usize];
                } else {
                    dp[i][j] = dp[i - 1][j];
                }
            }
        }

        dp[n][target]
    }
}
