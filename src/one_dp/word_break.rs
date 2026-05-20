use std::collections::HashMap;

pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
    let mut memo: HashMap<usize, bool> = HashMap::new();
    memo.insert(s.len(), true);
    dfs(&s, &word_dict, 0, &mut memo)
}

fn dfs(s: &String, word_dict: &Vec<String>, i: usize, memo: &mut HashMap<usize, bool>) -> bool {
    if memo.contains_key(&i) {
        return memo[&i];
    }

    for w in word_dict.iter() {
        if i + w.len() <= s.len() && &s[i..(i + w.len())] == w {
            if dfs(s, word_dict, i + w.len(), memo) {
                memo.insert(i, true);
                return true;
            }
        }
    }
    memo.insert(i, false);
    return false;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_word_break_case_1() {
        let s = String::from("neetcode");
        let word_dict = vec![String::from("neet"), String::from("code")];

        let result = word_break(s, word_dict);
        assert_eq!(result, true);
    }

    #[test]
    fn test_word_break_case_2() {
        let s = String::from("applepenapl");
        let word_dict = vec![
            String::from("apple"),
            String::from("pen"),
            String::from("app"),
        ];

        let result = word_break(s, word_dict);
        assert_eq!(result, false);
    }
}
