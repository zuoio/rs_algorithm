use std::collections::HashMap;

use crate::common::Solution;

impl Solution {
    pub fn network_delay_time(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
        let n = n as usize;
        let k = k as usize;

        let mut adj: HashMap<usize, Vec<(usize, i32)>> = HashMap::new();
        for time in &times {
            adj.entry(time[0] as usize)
                .or_default()
                .push((time[1] as usize, time[2]));
        }
        let mut dist = vec![i32::MAX; n + 1];

        fn dfs(
            node: usize,
            time: i32,
            adj: &HashMap<usize, Vec<(usize, i32)>>,
            dist: &mut Vec<i32>,
        ) {
            if time >= dist[node] {
                return;
            }
            dist[node] = time;
            if let Some(neighbors) = adj.get(&node) {
                for &(nei, w) in neighbors {
                    dfs(nei, time + w, adj, dist);
                }
            }
        }

        dfs(k, 0, &adj, &mut dist);
        let res = *dist[1..=n].iter().max().unwrap();
        if res == i32::MAX { -1 } else { res }
    }
}
