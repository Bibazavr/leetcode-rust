// https://leetcode.com/problems/as-far-from-land-as-possible/

struct Solution;

impl Solution {
    pub fn max_distance(grid: Vec<Vec<i32>>) -> i32 {
        use std::collections::VecDeque;
        let n = grid.len();

        let mut visited = vec![vec![false; n]; n];
        let mut distance = i32::MIN;

        let mut queue = VecDeque::new();
        for i in 0..n {
            for j in 0..n {
                if grid[i][j] == 1 {
                    queue.push_back((i as i32, j as i32, 0));
                    visited[i][j] = true;
                }
            }
        }

        while let Some((i, j, d)) = queue.pop_front() {
            distance = distance.max(d);
            for &(neighbor_x, neighbor_y) in [(i - 1, j), (i + 1, j), (i, j - 1), (i, j + 1)].iter()
            {
                if neighbor_x < 0
                    || neighbor_x >= n as i32
                    || neighbor_y < 0
                    || neighbor_y >= n as i32
                {
                    continue;
                }

                if !visited[neighbor_x as usize][neighbor_y as usize]
                    && grid[neighbor_x as usize][neighbor_y as usize] == 0
                {
                    visited[neighbor_x as usize][neighbor_y as usize] = true;
                    queue.push_back((neighbor_x, neighbor_y, d + 1));
                }
            }
        }

        if distance == 0 {
            -1
        } else {
            distance
        }
    }
}

#[allow(dead_code)]
pub fn main() {
    let n = vec![vec![1, 0, 1], vec![0, 0, 0], vec![1, 0, 1]];
    assert_eq!(Solution::max_distance(n), 2);

    let n = vec![vec![1, 0, 0], vec![0, 0, 0], vec![0, 0, 0]];
    assert_eq!(Solution::max_distance(n), 4);

    let n = vec![
        vec![1, 1, 1, 1, 1],
        vec![1, 1, 1, 1, 1],
        vec![1, 1, 1, 1, 1],
        vec![1, 1, 1, 1, 1],
        vec![1, 1, 1, 1, 1],
    ];
    assert_eq!(Solution::max_distance(n), -1);
}
