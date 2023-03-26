// https://leetcode.com/problems/longest-cycle-in-a-graph/
struct Solution;

impl Solution {
    fn dfs(
        vertex: usize,
        mut counter: i32,
        edges: &[i32],
        count_visited: &mut [i32],
        visited: &mut [bool],
    ) -> i32 {
        count_visited[vertex] = counter;
        visited[vertex] = true;

        counter += 1;

        let next_vertex = edges[vertex];

        if next_vertex == -1 {
            return -1;
        }

        let next_vertex = next_vertex as usize;
        if visited[next_vertex] && count_visited[next_vertex] != -1 {
            return counter - count_visited[next_vertex];
        }

        let res = if !visited[next_vertex] {
            Self::dfs(next_vertex, counter, edges, count_visited, visited)
        } else {
            -1
        };

        count_visited[next_vertex] = -1;

        res
    }

    pub fn longest_cycle(edges: Vec<i32>) -> i32 {
        let size = edges.len();

        (0..size)
            .fold(
                (-1, vec![-1; size], vec![false; size]),
                |(mut longest_cycle, mut count_visited, mut visited), node| {
                    if !visited[node] {
                        longest_cycle = std::cmp::max(
                            Self::dfs(node, 0, &edges, &mut count_visited, &mut visited),
                            longest_cycle,
                        );

                        count_visited[node] = -1;
                    }

                    (longest_cycle, count_visited, visited)
                },
            )
            .0
    }
}

#[allow(dead_code)]
pub fn main() {
    let edges = vec![3, 3, 4, 2, 3];
    assert_eq!(Solution::longest_cycle(edges), 3);
}
