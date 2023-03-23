// https://leetcode.com/problems/number-of-operations-to-make-network-connected/

struct Solution;

impl Solution {
    pub fn make_connected(n: i32, connections: Vec<Vec<i32>>) -> i32 {
        let size = n as usize;

        if connections.len() < size - 1 {
            return -1;
        }

        let mut graph = vec![vec![]; size];

        for connection in connections {
            graph[connection[0] as usize].push(connection[1] as usize);
            graph[connection[1] as usize].push(connection[0] as usize);
        }

        let mut visited = vec![false; size];

        use std::collections::VecDeque;
        let mut queue = VecDeque::new();
        let mut cmp_count = 0;

        for i in 0..size {
            queue.push_back(i);
            if visited[i] {
                continue;
            }

            cmp_count += 1;
            while let Some(idx) = queue.pop_front() {
                for &near in &graph[idx] {
                    if !visited[near] {
                        visited[near] = true;
                        queue.push_back(near);
                    }
                }
            }
        }
        cmp_count - 1
    }
}

#[allow(dead_code)]
pub fn main() {
    let n = 4;
    let connections = vec![vec![0, 1], vec![0, 2], vec![1, 2]];
    assert_eq!(Solution::make_connected(n, connections), 1)
}
