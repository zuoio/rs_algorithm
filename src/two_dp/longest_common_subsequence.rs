use crate::common::Solution;

impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let t1 = text1.as_bytes();
        let t2 = text2.as_bytes();
        let mut memo = vec![vec![-1i32; t2.len()]; t1.len()];

        fn dfs(t1: &[u8], t2: &[u8], i: usize, j: usize, memo: &mut Vec<Vec<i32>>) -> i32 {
            if i == t1.len() || j == t2.len() {
                return 0;
            }
            if memo[i][j] != -1 {
                return memo[i][j];
            }
            if t1[i] == t2[j] {
                memo[i][j] = 1 + dfs(t1, t2, i + 1, j + 1, memo);
            } else {
                memo[i][j] = dfs(t1, t2, i + 1, j, memo).max(dfs(t1, t2, i, j + 1, memo));
            }
            memo[i][j]
        }
        dfs(t1, t2, 0, 0, &mut memo)
    }
}
