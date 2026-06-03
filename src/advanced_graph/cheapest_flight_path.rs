use std::{cmp::Reverse, collections::BinaryHeap};

use crate::common::Solution;

impl Solution {
    pub fn find_cheapest_price(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
        let mut adj = vec![vec![]; n as usize];
        let mut dist = vec![vec![i32::MAX; (k + 2) as usize]; n as usize];

        for flight in &flights {
            adj[flight[0] as usize].push((flight[1] as usize, flight[2]));
        }
        dist[src as usize][0] = 0;

        let mut min_heap = BinaryHeap::new();
        min_heap.push(Reverse((0, src as usize, -1i32)));

        while let Some(Reverse((cost, city, stops))) = min_heap.pop() {
            if city == dst as usize {
                return cost;
            }
            if stops == k || dist[city][(stops + 1) as usize] < cost {
                continue;
            }
            for &(nei, w) in &adj[city] {
                let next_cost = cost + w;
                let next_stops = stops + 1;
                if dist[nei][(next_stops + 1) as usize] > next_cost {
                    dist[nei][(next_stops + 1) as usize] = next_cost;
                    min_heap.push(Reverse((next_cost, nei, next_stops)));
                }
            }
        }
        -1
    }
}
