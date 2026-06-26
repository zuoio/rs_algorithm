use crate::common::Solution;

impl Solution {
    pub fn max_coins(nums: Vec<i32>) -> i32 {
        let mut new_nums = vec![1];
        new_nums.extend_from_slice(&nums);
        new_nums.push(1);
        let n = new_nums.len();
        let mut dp = vec![vec![-1i32; n]; n];

        fn dfs(l: usize, r: usize, dp: &mut Vec<Vec<i32>>, nums: &[i32]) -> i32 {
            if l > r {
                return 0;
            }
            if dp[l][r] != -1 {
                return dp[l][r];
            }

            let mut res = 0;
            for i in l..=r {
                let mut sum = 0;
                sum += nums[l - 1] * nums[i] * nums[r + 1];
                sum += dfs(l, i - 1, dp, nums);
                sum += dfs(i + 1, r, dp, nums);
                res = res.max(sum);
            }
            dp[l][r] = res;
            res
        }

        dfs(1, n, &mut dp, &nums)
    }
}
