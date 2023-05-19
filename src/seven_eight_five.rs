// https://leetcode.com/problems/is-graph-bipartite/

struct Solution;

impl Solution {
    pub fn is_bipartite(graph: Vec<Vec<i32>>) -> bool {
        let n = graph.len();

        let mut colors = vec![0; n];
        let mut q = Vec::with_capacity(n);
        let mut head = 0;

        for i in 0..n {
            if colors[i] != 0 {
                continue;
            }
            colors[i] = 1;
            q.push(i);
            while head < q.len() {
                let v = q[head] as usize;
                for &u in &graph[v] {
                    let u = u as usize;
                    if colors[u] == 0 {
                        colors[u] = 3 - colors[v];
                        q.push(u);
                    } else if colors[u] == colors[v] {
                        return false;
                    }
                }
                head += 1;
            }
        }

        true
    }
}

#[allow(dead_code)]
pub fn main() {
    let graph = vec![vec![1, 2, 3], vec![0, 2], vec![0, 1, 3], vec![0, 2]];

    assert_eq!(Solution::is_bipartite(graph), false);
}
