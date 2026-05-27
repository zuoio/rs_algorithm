use crate::common::Solution;

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits.is_empty() {
            return vec![];
        }

        let digits_to_char: [&str; 10] = [
            "", "", "abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz",
        ];

        let mut res = vec![String::new()];

        for b in digits.bytes() {
            let letters = digits_to_char[(b - b'0') as usize];
            let mut temp = Vec::new();
            for cur in &res {
                for c in letters.chars() {
                    let mut s = cur.clone();
                    s.push(c);
                    temp.push(s);
                }
            }
            res = temp;
        }

        res
    }
}
