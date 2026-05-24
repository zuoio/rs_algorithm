use crate::common::Solution;

impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let rows = board.len();
        let cols = board[0].len();

        let word: Vec<char> = word.chars().collect();
        let mut visited = vec![false; rows];

        for r in 0..rows {
            for c in 0..cols {
                if Self::dfs(&board, &word, r as i32, c as i32, 0, &mut visited) {
                    return true;
                }
            }
        }

        return false;
    }

    fn dfs(
        board: &Vec<Vec<char>>,
        word: &[char],
        r: i32,
        c: i32,
        i: usize,
        visited: &mut Vec<bool>,
    ) -> bool {
        if i == word.len() {
            return true;
        }

        if r < 0 || c < 0 || r >= board.len() as i32 || c >= board[0].len() as i32 {
            return false;
        }

        let (ru, cu) = (r as usize, c as usize);
        let path = ru * board[0].len() + cu;
        if board[ru][cu] != word[i] || visited[path] {
            return false;
        }

        visited[path] = true;

        let res = Self::dfs(board, word, r + 1, c, i + 1, visited)
            || Self::dfs(board, word, r - 1, c, i + 1, visited)
            || Self::dfs(board, word, r, c + 1, i + 1, visited)
            || Self::dfs(board, word, r, c - 1, i + 1, visited);
        visited[path] = false;
        res
    }
}
