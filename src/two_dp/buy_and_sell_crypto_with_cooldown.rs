use crate::common::Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        // Self::recursion(prices)
        Self::top_down(prices)
    }

    fn recursion(prices: Vec<i32>) -> i32 {
        fn dfs(i: usize, buying: bool, prices: &[i32]) -> i32 {
            if i >= prices.len() {
                return 0;
            }

            let cooldown = dfs(i + 1, buying, prices);

            if buying {
                let buy = dfs(i + 1, false, prices) - prices[i];
                buy.max(cooldown)
            } else {
                let sell = dfs(i + 2, true, prices) + prices[i];
                sell.max(cooldown)
            }
        }
        dfs(0, true, &prices)
    }

    fn top_down(prices: Vec<i32>) -> i32 {
        let n = prices.len();
        let mut dp = vec![vec![-1i32; 2]; n];

        fn dfs(i: usize, buying: usize, prices: &[i32], dp: &mut Vec<Vec<i32>>) -> i32 {
            if i >= prices.len() {
                return 0;
            }
            if dp[i][buying] != -1 {
                return dp[i][buying];
            }
            let cooldown = dfs(i + 1, buying, prices, dp);
            dp[i][buying] = if buying == 1 {
                let buy = dfs(i + 1, 0, prices, dp) - prices[i];
                buy.max(cooldown)
            } else {
                let sell = dfs(i + 2, 1, prices, dp) + prices[i];
                sell.max(cooldown)
            };

            dp[i][buying]
        }
        dfs(0, 1, &prices, &mut dp)
    }

    fn bottom_up(prices: Vec<i32>) -> i32 {
        let n = prices.len();
        let mut dp = vec![vec![0i32; 2]; n + 1];

        for i in (0..n).rev() {
            let buy = dp[i + 1][0] - prices[i];
            let cooldown = dp[i + 1][1];
            dp[i][1] = buy.max(cooldown);

            let sell = if i + 1 < n {
                dp[i + 2][1] + prices[i]
            } else {
                prices[i]
            };
            let cooldown = dp[i + 1][0];
            dp[i][0] = sell.max(cooldown);
        }
        dp[0][1]
    }
}
