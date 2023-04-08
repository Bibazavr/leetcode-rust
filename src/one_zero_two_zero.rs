// https://leetcode.com/problems/number-of-enclaves/

struct Solution;

impl Solution {
    pub fn num_enclaves(mut grid: Vec<Vec<i32>>) -> i32 {
        let mut count = 0;
        let (n, m) = (grid.len() - 1, grid[0].len() - 1);
        for i in 0..=n {
            for j in 0..=m {
                if grid[i][j] == 1 {
                    count += Self::dfs(&mut grid, i, j, n, m);
                }
            }
        }
        count
    }
    fn dfs(grid: &mut Vec<Vec<i32>>, i: usize, j: usize, n: usize, m: usize) -> i32 {
        let mut queue = std::collections::VecDeque::new();
        queue.push_front((i, j));
        let mut count = 0;
        let mut border = false;

        while let Some((i, j)) = queue.pop_front() {
            if grid[i][j] != 1 {
                continue;
            }

            count += 1;
            grid[i][j] = -1;

            if i == n || i == 0 || j == 0 || j == m {
                border = true;
            }

            if i > 0 && grid[i - 1][j] == 1 {
                queue.push_back((i - 1, j));
            }
            if i < n && grid[i + 1][j] == 1 {
                queue.push_back((i + 1, j));
            }
            if j > 0 && grid[i][j - 1] == 1 {
                queue.push_back((i, j - 1));
            }
            if j < m && grid[i][j + 1] == 1 {
                queue.push_back((i, j + 1));
            }
        }

        match border {
            true => 0,
            false => count,
        }
    }
}

#[allow(dead_code)]
pub fn main() {
    let grid = vec![
        vec![0, 1, 1, 0],
        vec![0, 0, 1, 0],
        vec![0, 0, 1, 0],
        vec![0, 0, 0, 0],
    ];

    assert_eq!(Solution::num_enclaves(grid), 0);

    let grid = vec![
        vec![0, 0, 0, 0],
        vec![1, 0, 1, 0],
        vec![0, 1, 1, 0],
        vec![0, 0, 0, 0],
    ];
    assert_eq!(Solution::num_enclaves(grid), 3);
}
