use std::collections::HashMap;

use crate::common::Solution;

impl Solution {
    pub fn find_itinerary(tickets: Vec<Vec<String>>) -> Vec<String> {
        let mut adj: HashMap<String, Vec<String>> = HashMap::new();
        for ticket in tickets.iter() {
            let from = ticket[0].clone();
            let to = ticket[1].clone();
            adj.entry(from).or_insert_with(Vec::new).push(to);
        }

        for destinations in adj.values_mut() {
            destinations.sort();
        }

        let mut route = Vec::new();
        fn dfs(airport: &str, adj: &mut HashMap<String, Vec<String>>, route: &mut Vec<String>) {
            while let Some(destinations) = adj.get_mut(airport) {
                if destinations.is_empty() {
                    break;
                }
                let next = destinations.remove(0);
                dfs(&next, adj, route);
            }
            route.push(airport.to_string());
        }
        dfs("JFK", &mut adj, &mut route);
        route.reverse();
        route
    }
}
