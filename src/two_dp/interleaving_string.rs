use std::vec;

use crate::common::Solution;

impl Solution {
    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        // Self::recurse(s1, s2, s3)
        Self::dp(s1, s2, s3)
    }

    fn recurse(s1: String, s2: String, s3: String) -> bool {
        if s1.len() + s2.len() != s3.len() {
            return false;
        }
        fn dfs(i: usize, j: usize, k: usize, s1: &[u8], s2: &[u8], s3: &[u8]) -> bool {
            if k == s3.len() {
                return true;
            }
            if i >= s1.len() && j >= s2.len() && k < s3.len() - 1 {
                // s1、s2 无法组成 s3
                return false;
            }

            let mut res = false;
            if i < s1.len() && s1[i] == s3[k] {
                res = res || dfs(i + 1, j, k + 1, s1, s2, s3);
            }
            if j < s2.len() && s2[j] == s3[k] {
                res = res || dfs(i, j + 1, k + 1, s1, s2, s3);
            }
            res
        }
        let (s1, s2, s3) = (s1.as_bytes(), s2.as_bytes(), s3.as_bytes());
        dfs(0, 0, 0, s1, s2, s3)
    }

    fn dp(s1: String, s2: String, s3: String) -> bool {
        if s1.len() + s2.len() != s3.len() {
            return false;
        }
        // 由于 k = i + j, 因此我们可以将 （i， j）当作一个状态进行存储
        let (s1, s2, s3) = (s1.as_bytes(), s2.as_bytes(), s3.as_bytes());
        let (l1, l2, l3) = (s1.len(), s2.len(), s3.len());

        let mut dp = vec![vec![-1i32; l2 + 1]; l1 + 1];

        fn dfs(
            i: usize,
            j: usize,
            k: usize,
            dp: &mut Vec<Vec<i32>>,
            s1: &[u8],
            s2: &[u8],
            s3: &[u8],
            l1: usize,
            l2: usize,
            l3: usize,
        ) -> bool {
            if i + j == l3 {
                return true;
            }
            if i >= l1 && j >= l2 && k < l3 - 1 {
                return false;
            }
            if dp[i][j] != -1 {
                return dp[i][j] == 1;
            }

            let mut res = false;
            if i < l1 && s1[i] == s3[k] {
                res = res || dfs(i + 1, j, k + 1, dp, s1, s2, s3, l1, l2, l3);
            }
            if j < l2 && s2[j] == s3[k] {
                res = res || dfs(i, j + 1, k + 1, dp, s1, s2, s3, l1, l2, l3);
            }
            dp[i][j] = if res { 1 } else { 0 };
            res
        }
        dfs(0, 0, 0, &mut dp, s1, s2, s3, l1, l2, l3)
    }
}

#[cfg(test)]
mod test {
    use crate::common::Solution;

    #[test]
    fn is_interleave_test() {
        let s1 = String::from("aaaa");
        let s2 = String::from("bbbb");
        let s3 = String::from("aabbbbaa");

        let solution = Solution::is_interleave(s1, s2, s3);
        assert_eq!(solution, true);
    }
}
