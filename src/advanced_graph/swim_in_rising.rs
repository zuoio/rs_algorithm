use std::{cmp::Reverse, collections::BinaryHeap};

use crate::common::Solution;

impl Solution {
    pub fn swim_in_water(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut visited = vec![vec![false; n]; n];
        let mut heap = BinaryHeap::new();
        let directions: [(isize, isize); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];
        heap.push(Reverse((grid[0][0], 0usize, 0usize)));
        visited[0][0] = true;

        while let Some(Reverse((t, r, c))) = heap.pop() {
            if r == n - 1 && c == n - 1 {
                return t;
            }

            for &(dr, dc) in &directions {
                let Some(nr) = r.checked_add_signed(dr) else {
                    continue;
                };
                let Some(nc) = c.checked_add_signed(dc) else {
                    continue;
                };

                if nr < n && nc < n {
                    let (nr, nc) = (nr as usize, nc as usize);
                    if !visited[nr][nc] {
                        visited[nr][nc] = true;
                        heap.push(Reverse((t.max(grid[nr][nc]), nr, nc)));
                    }
                }
            }
        }

        return n as i32 * n as i32;
    }
}
