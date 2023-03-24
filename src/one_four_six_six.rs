// https://leetcode.com/problems/reorder-routes-to-make-all-paths-lead-to-the-city-zero/

struct Solution;

impl Solution {
    pub fn min_reorder(n: i32, connections: Vec<Vec<i32>>) -> i32 {
        let size = n as usize;
        let mut count: i32 = 0;

        let mut graph = vec![vec![]; size];
        let mut visited = vec![false; size];

        for connection in connections {
            graph[connection[0] as usize].push((connection[1] as usize, 1));
            graph[connection[1] as usize].push((connection[0] as usize, 0));
        }

        let mut queue = std::collections::VecDeque::new();

        queue.push_front((0, 0));

        while let Some((next, direction)) = queue.pop_front() {
            count += direction;
            visited[next] = true;
            for &(to, d) in &graph[next] {
                if !visited[to] {
                    queue.push_back((to, d));
                }
            }
        }
        count
    }
}

#[allow(dead_code)]
pub fn main() {
    let n = 6;
    let connections = vec![vec![0, 1], vec![1, 3], vec![2, 3], vec![4, 0], vec![4, 5]];
    assert_eq!(Solution::min_reorder(n, connections), 3);
}
