use crate::common::Solution;

impl Solution {
    pub fn num_distinct(s: String, t: String) -> i32 {
        let s = s.as_bytes();
        let t = t.as_bytes();
        let m = s.len();
        let n = t.len();
        if n > m {
            return 0;
        }

        let mut dp = vec![vec![-1i32; n + 1]; m + 1];
        fn dfs(
            i: usize,
            j: usize,
            m: usize,
            n: usize,
            dp: &mut Vec<Vec<i32>>,
            s: &[u8],
            t: &[u8],
        ) -> i32 {
            if j == n {
                return 1;
            }
            if i == m {
                return 0;
            }
            if dp[i][j] != -1 {
                return dp[i][j];
            }
            let mut res = 0;
            res += dfs(i + 1, j, m, n, dp, s, t);
            if s[i] == t[j] {
                res += dfs(i + 1, j + 1, m, n, dp, s, t);
            }
            dp[i][j] = res;
            res
        }

        dfs(0, 0, m, n, &mut dp, s, t)
    }
}
