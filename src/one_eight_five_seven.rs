// https://leetcode.com/problems/largest-color-value-in-a-directed-graph/

struct Solution;

impl Solution {
    pub fn largest_path_value(colors: String, edges: Vec<Vec<i32>>) -> i32 {
        let n = colors.len();
        let mut graph = vec![vec![]; n];
        let mut degree = vec![0; n];

        for pair in edges {
            graph[pair[0] as usize].push(pair[1] as usize);
            degree[pair[1] as usize] += 1;
        }

        let mut queue = std::collections::VecDeque::new();
        for i in 0..n {
            if degree[i] == 0 {
                queue.push_back(i);
            }
        }

        let mut dp = vec![[0; 26]; n];
        let mut result = 1;
        while let Some(i) = queue.pop_front() {
            let c = (colors.as_bytes()[i] - b'a') as usize;
            dp[i][c] += 1;
            result = result.max(*dp[i].iter().max().unwrap_or(&-1));

            for &j in graph[i].iter() {
                degree[j] -= 1;
                for c in 0..26 {
                    dp[j][c] = dp[j][c].max(dp[i][c]);
                }
                if degree[j] == 0 {
                    queue.push_back(j);
                }
            }
        }
        if degree.iter().any(|&d| d > 0) {
            return -1;
        }
        result
    }
}

#[allow(dead_code)]
pub fn main() {
    let colors = String::from("abaca");
    let edges = vec![vec![0, 1], vec![0, 2], vec![2, 3], vec![3, 4]];

    assert_eq!(Solution::largest_path_value(colors, edges), 3)
}
