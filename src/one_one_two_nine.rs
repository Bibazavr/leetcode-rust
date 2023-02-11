// https://leetcode.com/problems/shortest-path-with-alternating-colors/

struct Solution;

impl Solution {
    pub fn shortest_alternating_paths(
        n: i32,
        red_edges: Vec<Vec<i32>>,
        blue_edges: Vec<Vec<i32>>,
    ) -> Vec<i32> {
        use std::collections::VecDeque;
        let n = n as usize;

        let mut red_graph = vec![vec![]; n];
        let mut blue_graph = vec![vec![]; n];
        for edge in red_edges {
            red_graph[edge[0] as usize].push(edge[1] as usize);
        }
        for edge in blue_edges {
            blue_graph[edge[0] as usize].push(edge[1] as usize);
        }

        let mut visited_red = vec![false; n];
        let mut visited_blue = vec![false; n];
        visited_red[0] = true;
        visited_blue[0] = true;

        let mut queue = VecDeque::new();

        queue.push_back((0, true, 0));
        queue.push_back((0, false, 0));

        let mut result = vec![-1; n];

        while let Some((node, is_red, distance)) = queue.pop_front() {
            if result[node] == -1 {
                result[node] = distance;
            }

            match is_red {
                true => {
                    for &blue_node in &blue_graph[node] {
                        if visited_blue[blue_node] == false {
                            queue.push_back((blue_node, false, distance + 1));
                            visited_blue[blue_node] = true;
                        }
                    }
                }
                false => {
                    for &red_node in &red_graph[node] {
                        if visited_red[red_node] == false {
                            queue.push_back((red_node, true, distance + 1));
                            visited_red[red_node] = true;
                        }
                    }
                }
            }
        }
        result
    }
}

#[allow(dead_code)]
pub fn main() {
    let n = 3;
    let red_edges = vec![vec![0, 1], vec![0, 2]];
    let blue_edges = vec![vec![1, 0]];
    assert_eq!(
        Solution::shortest_alternating_paths(n, red_edges, blue_edges),
        vec![0, 1, 1]
    );

    let n = 3;
    let red_edges = vec![vec![0, 1], vec![1, 2]];
    let blue_edges = vec![];
    assert_eq!(
        Solution::shortest_alternating_paths(n, red_edges, blue_edges),
        vec![0, 1, -1]
    );

    let n = 3;
    let red_edges = vec![vec![0, 1]];
    let blue_edges = vec![vec![2, 1]];
    assert_eq!(
        Solution::shortest_alternating_paths(n, red_edges, blue_edges),
        vec![0, 1, -1]
    );
}
