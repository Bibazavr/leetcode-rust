// https://leetcode.com/problems/minimum-time-to-collect-all-apples-in-a-tree/description/

struct Solution;

impl Solution {
    pub fn min_time(n: i32, edges: Vec<Vec<i32>>, has_apple: Vec<bool>) -> i32 {
        let mut graph = vec![vec![]; n as usize];
        for edge in edges.iter() {
            let a = edge[0];
            let b = edge[1];
            graph[a as usize].push(b as usize);
            graph[b as usize].push(a as usize);
        }

        return Solution::dfs(0, 0, &graph, &has_apple);
    }

    fn dfs(prev: usize, curr: usize, graph: &Vec<Vec<usize>>, has_apple: &Vec<bool>) -> i32 {
        let mut ans = 0;
        for &i in graph[curr].iter() {
            if i != prev {
                let ans2 = Self::dfs(curr, i, graph, has_apple);
                ans += ans2;
                if ans2 > 0 || has_apple[i] {
                    ans += 2;
                }
            }
        }
        ans
    }
}

#[allow(dead_code)]
pub fn main() {
    let n = 7;

    let edges = vec![
        vec![0, 1],
        vec![0, 2],
        vec![1, 4],
        vec![1, 5],
        vec![2, 3],
        vec![2, 6],
    ];

    let has_apple = vec![false, false, true, false, true, true, false];

    assert_eq!(Solution::min_time(n, edges, has_apple), 8);
}
