use std::collections::HashMap;

use crate::common::Solution;

impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        let s = s.as_bytes();
        let n = s.len();
        let mut dp = HashMap::new();
        dp.insert(n, 1);

        fn dfs(s: &[u8], i: usize, dp: &mut HashMap<usize, i32>) -> i32 {
            if let Some(&val) = dp.get(&i) {
                return val;
            }

            if s[i] == b'0' {
                return 0;
            }

            let mut res = dfs(s, i + 1, dp);
            if i + 1 < s.len() && (s[i] == b'1' || (s[i] == b'2' && s[i + 1] < b'7')) {
                res += dfs(s, i + 2, dp);
            }
            dp.insert(i, res);
            res
        }

        dfs(s, 0, &mut dp)
    }
}
