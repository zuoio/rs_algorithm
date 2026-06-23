use crate::common::Solution;

impl Solution {
    pub fn longest_increasing_path(matrix: Vec<Vec<i32>>) -> i32 {
        let r = matrix.len();
        let c = matrix[0].len();

        let mut dp = vec![vec![-1i32; c]; r];

        fn dfs(
            row: i32,
            col: i32,
            prev: i32,
            dp: &mut Vec<Vec<i32>>,
            matrix: &Vec<Vec<i32>>,
            r: i32,
            c: i32,
        ) -> i32 {
            if row < 0
                || col < 0
                || row >= r
                || col >= c
                || matrix[row as usize][col as usize] <= prev
            {
                return 0;
            }
            let (row, col) = (row as usize, col as usize);
            if dp[row][col] != -1 {
                return dp[row][col];
            }

            let mut res = 0;
            let dir: Vec<(i32, i32)> = vec![(1, 0), (0, 1), (-1, 0), (0, -1)];
            for (x, y) in dir {
                res = dfs(
                    row as i32 + x,
                    col as i32 + y,
                    matrix[row][col],
                    dp,
                    matrix,
                    r,
                    c,
                )
                .max(res);
            }
            res += 1;
            dp[row][col] = res;
            return res;
        }

        let mut res = 0;
        for i in 0..r {
            for j in 0..c {
                res = dfs(i as i32, j as i32, -1, &mut dp, &matrix, r as i32, c as i32).max(res);
            }
        }
        res
    }
}
