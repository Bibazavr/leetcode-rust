// https://leetcode.com/problems/minimum-path-sum/

struct Solution;

impl Solution {
    pub fn min_path_sum(mut grid: Vec<Vec<i32>>) -> i32 {
        if grid.is_empty() {
            return 0;
        }
        let (m, n) = (grid.len(), grid[0].len());
        for i in 0..m {
            for j in 0..n {
                if i == 0 && j == 0 {
                    continue;
                } else if i == 0 {
                    grid[i][j] += grid[i][j - 1];
                } else if j == 0 {
                    grid[i][j] += grid[i - 1][j];
                } else {
                    grid[i][j] += std::cmp::min(grid[i][j - 1], grid[i - 1][j]);
                }
            }
        }
        grid[m - 1][n - 1]
    }
}

#[allow(dead_code)]
pub fn main() {
    let grid = vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]];
    assert_eq!(Solution::min_path_sum(grid), 7);
}
