use crate::common::Solution;

impl Solution {
    pub fn partition(s: String) -> Vec<Vec<String>> {
        let s = s.chars().collect();
        let mut res: Vec<Vec<String>> = Vec::new();
        let mut part: Vec<String> = Vec::new();

        fn is_palindrome(s: &Vec<char>, mut l: usize, mut r: usize) -> bool {
            while l < r {
                if s[l] != s[r] {
                    return false;
                }
                l += 1;
                r -= 1;
            }
            true
        }

        fn dfs(s: &Vec<char>, res: &mut Vec<Vec<String>>, part: &mut Vec<String>, i: usize) {
            if i >= s.len() {
                res.push(part.clone());
            }

            for j in i..s.len() {
                if is_palindrome(s, i, j) {
                    part.push(s[i..=j].iter().collect());
                    dfs(s, res, part, j + 1);
                    part.pop();
                }
            }
        }

        dfs(&s, &mut res, &mut part, 0);
        res
    }
}
