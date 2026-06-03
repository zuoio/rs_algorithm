use std::{cmp::Reverse, collections::BinaryHeap, vec};

use crate::common::Solution;

impl Solution {
    pub fn min_cost_to_connect_points(points: Vec<Vec<i32>>) -> i32 {
        let len = points.len();
        let mut adj: Vec<Vec<(i32, usize)>> = vec![vec![]; len];

        for i in 0..len {
            for j in 0..len {
                let dist =
                    (points[i][0] - points[j][0]).abs() + (points[i][0] - points[j][0]).abs();
                adj[i].push((dist, j));
                adj[j].push((dist, i));
            }
        }

        let mut res = 0;
        let mut visit = vec![false; len];
        let mut visited_count = 0;
        let mut min_heap = BinaryHeap::new();
        min_heap.push(Reverse((0, 0 as usize)));

        while visited_count < len {
            let Reverse((cost, i)) = min_heap.pop().unwrap();
            if visit[i] {
                continue;
            }
            res += cost;
            visit[i] = true;
            visited_count += 1;
            for &(nei_cost, nei) in &adj[i] {
                if !visit[nei] {
                    min_heap.push(Reverse((nei_cost, nei)));
                }
            }
        }
        return res;
    }
}
