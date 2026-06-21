use crate::common::Solution;

impl Solution {
    pub fn change(amount: i32, coins: Vec<i32>) -> i32 {
        let mut coins = coins;
        coins.sort();
        let n = coins.len();
        let a = amount as usize;
        let mut memo = vec![vec![-1i32; a + 1]; n + 1];

        fn dfs(coins: &[i32], memo: &mut Vec<Vec<i32>>, i: usize, a: i32) -> i32 {
            if a == 0 {
                return 1;
            }
            if i >= coins.len() {
                return 0;
            }
            if memo[i][a as usize] != -1 {
                return memo[i][a as usize];
            }
            let mut res = 0;
            if a >= coins[i] {
                res = dfs(coins, memo, i + 1, a) + dfs(coins, memo, i, a - coins[i]);
            }
            memo[i][a as usize] = res;
            res
        }

        dfs(&coins, &mut memo, 0, amount)
    }
}
