// https://leetcode.com/problems/minimum-score-of-a-path-between-two-cities/

struct Solution;

impl Solution {
    pub fn min_score(n: i32, roads: Vec<Vec<i32>>) -> i32 {
        let mut min_score = i32::MAX;

        let size = 1 + n as usize;

        let mut graph = vec![vec![]; size];

        for road in roads {
            graph[road[0] as usize].push((road[1], road[2]));
            graph[road[1] as usize].push((road[0], road[2]));
        }

        let mut stack = vec![1];
        let mut visit = vec![false; size];
        visit[1] = true;

        while let Some(next) = stack.pop() {
            for &(node, distance) in &graph[next as usize] {
                min_score = min_score.min(distance);

                if !visit[node as usize] {
                    visit[node as usize] = true;
                    stack.push(node);
                }
            }
        }
        min_score
    }
}

#[allow(dead_code)]
pub fn main() {
    let roads = vec![vec![1, 2, 9], vec![2, 3, 6], vec![2, 4, 5], vec![1, 4, 7]];
    let n = 5;
    assert_eq!(Solution::min_score(n, roads), 5);
}
