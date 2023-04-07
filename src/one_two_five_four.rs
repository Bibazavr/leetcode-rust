// https://leetcode.com/problems/number-of-closed-islands/

struct Solution;

impl Solution {
    pub fn closed_island(mut grid: Vec<Vec<i32>>) -> i32 {
        let mut count = 0;
        let (n, m) = (grid.len() - 1, grid[0].len() - 1);
        for i in 1..=n {
            for j in 1..=m {
                if grid[i][j] == 0 {
                    count += Self::dfs(&mut grid, i, j, n, m);
                }
            }
        }
        count
    }

    fn dfs(grid: &mut Vec<Vec<i32>>, i: usize, j: usize, n: usize, m: usize) -> i32 {
        let mut queue = std::collections::VecDeque::new();
        queue.push_front((i, j));
        let mut count = 1;

        while let Some((i, j)) = queue.pop_front() {
            if grid[i][j] != 0 {
                continue;
            }
            if i.min(j) == 0 || i == n || j == m {
                count = 0;
            }
            grid[i][j] = -1;

            if i > 0 && grid[i - 1][j] == 0 {
                queue.push_back((i - 1, j));
            }
            if i < n && grid[i + 1][j] == 0 {
                queue.push_back((i + 1, j));
            }
            if j > 0 && grid[i][j - 1] == 0 {
                queue.push_back((i, j - 1));
            }
            if j < m && grid[i][j + 1] == 0 {
                queue.push_back((i, j + 1));
            }
        }
        count
    }
}

#[allow(dead_code)]
pub fn main() {
    let grid = vec![
        vec![1, 1, 1, 1, 1, 1, 1, 0],
        vec![1, 0, 0, 0, 0, 1, 1, 0],
        vec![1, 0, 1, 0, 1, 1, 1, 0],
        vec![1, 0, 0, 0, 0, 1, 0, 1],
        vec![1, 1, 1, 1, 1, 1, 1, 0],
    ];
    assert_eq!(Solution::closed_island(grid), 2)
}
