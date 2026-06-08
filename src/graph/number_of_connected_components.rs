use crate::common::Solution;

impl Solution {
    pub fn count_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut graph = vec![vec![]; n];
        for e in &edges {
            let u = e[0] as usize;
            let v = e[1] as usize;
            graph[u].push(v);
            graph[v].push(u);
        }

        let mut visited = vec![false; n];
        let mut count = 0;

        fn dfs(graph: &[Vec<usize>], visited: &mut [bool], node: usize) {
            visited[node] = true;
            for &neighbor in &graph[node] {
                if !visited[neighbor] {
                    dfs(graph, visited, neighbor);
                }
            }
        }

        for i in 0..n {
            if !visited[i] {
                count += 1;
                dfs(&graph, &mut visited, i);
            }
        }

        count
    }
}

#[cfg(test)]
mod tests {
    use crate::common::Solution;

    #[test]
    fn count_components_test() {
        let res = Solution::count_components(5, vec![vec![0, 1], vec![1, 2], vec![3, 4]]);
        assert_eq!(2, res);
    }
}
