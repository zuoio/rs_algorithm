use std::collections::{HashMap, HashSet, VecDeque};

use crate::common::Solution;

impl Solution {
    pub fn ladder_length(bengin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        if !word_list.contains(&end_word) || bengin_word == end_word {
            return 0;
        }

        let n = word_list.len();
        let m = word_list[0].len();
        let mut adj: Vec<Vec<usize>> = vec![vec![]; n];
        let mut mp = HashMap::new();
        for i in 0..n {
            mp.insert(&word_list[i], i);
        }

        let words_bytes: Vec<&[u8]> = word_list.iter().map(|w| w.as_bytes()).collect();
        for i in 0..n {
            for j in (i + 1)..n {
                let cnt = (0..m)
                    .filter(|&k| words_bytes[i][k] != words_bytes[j][k])
                    .count();
                if cnt == 1 {
                    adj[i].push(j);
                    adj[j].push(i);
                }
            }
        }

        let mut q = VecDeque::new();
        let mut res = 1;
        let mut visit = HashSet::new();
        let begin_bytes = bengin_word.as_bytes();

        for i in 0..m {
            for c in b'a'..=b'z' {
                if c == begin_bytes[i] {
                    continue;
                }
                let mut word = bengin_word.clone().into_bytes();
                word[i] = c;
                let word = String::from_utf8(word).unwrap();
                if let Some(&idx) = mp.get(&word) {
                    if visit.insert(idx) {
                        q.push_back(idx);
                    }
                }
            }
        }

        while !q.is_empty() {
            res += 1;
            let size = q.len();
            for _ in 0..size {
                let node = q.pop_front().unwrap();
                if word_list[node] == end_word {
                    return res;
                }
                for &nei in &adj[node] {
                    if visit.insert(nei) {
                        q.push_back(nei);
                    }
                }
            }
        }
        0
    }
}
