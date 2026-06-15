use crate::common::Solution;

impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let (m, n) = (m as usize, n as usize);
        let mut memo = vec![vec![-1i32; n]; m];

        fn dfs(i: usize, j: usize, m: usize, n: usize, memo: &mut Vec<Vec<i32>>) -> i32 {
            if i == m - 1 && j == n - 1{
                return 1;
            }
            if i >= m || j >= n {
                return 0;
            }
            if memo[i][j] != -1{
                return memo[i][j];
            }
            memo[i][j] = dfs(i, j + 1, m, n, memo) + dfs(i + 1, j, m, n, memo);
            memo[i][j]
        }

        dfs(0, 0, m, n, &mut memo)
    }
}
