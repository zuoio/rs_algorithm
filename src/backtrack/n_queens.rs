use crate::common::Solution;

impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        // 初始化一个大小为 n x n 的棋盘，其中 ‘Q’ 表示皇后， ‘.’表示空位
        let n = n as usize;
        let mut state: Vec<Vec<u8>> = vec![vec![b'.'; n]; n];

        // 记录列是否有皇后
        let mut cols = vec![false; n];
        // 记录主对角线上是否有皇后
        let mut diags1 = vec![false; 2 * n - 1];

        // 记录次对角线上是否有皇后
        let mut diags2 = vec![false; 2 * n - 1];
        let mut res: Vec<Vec<String>> = Vec::new();

        Self::bracktrack(
            0,
            n,
            &mut state,
            &mut res,
            &mut cols,
            &mut diags1,
            &mut diags2,
        );

        return res;
    }

    fn bracktrack(
        row: usize,
        n: usize,
        state: &mut Vec<Vec<u8>>,
        res: &mut Vec<Vec<String>>,
        cols: &mut Vec<bool>,
        diags1: &mut Vec<bool>,
        diags2: &mut Vec<bool>,
    ) {
        // 当放置完所有行时，记录解
        if row == n {
            res.push(
                state
                    .iter()
                    .map(|row| String::from_utf8(row.clone()).unwrap())
                    .collect(),
            );
            return;
        }

        // 遍历所有列
        for col in 0..n {
            // 计算该格子对应的主对角线和次对角线
            let diag1 = row - col + n - 1;
            let diag2 = row + col;
            // 剪枝：不允许该格子所在列、主对角线、次对角线上存在皇后
            if !cols[col] && !diags1[diag1] && !diags2[diag2] {
                // 尝试放置皇后
                state[row][col] = b'Q';
                cols[col] = true;
                diags1[diag1] = true;
                diags2[diag2] = true;
                // 放置下一行
                Self::bracktrack(row + 1, n, state, res, cols, diags1, diags2);
                // 回退: 将格子恢复为空位
                state[row][col] = b'.';
                cols[col] = false;
                diags1[diag1] = false;
                diags2[diag2] = false;
            }
        }
    }
}
