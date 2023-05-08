// https://leetcode.com/problems/matrix-diagonal-sum/

struct Solution;

impl Solution {
    pub fn diagonal_sum(mat: Vec<Vec<i32>>) -> i32 {
        let (mut result, n) = (0, mat.len());
        for i in 0..n {
            result += mat[i][i] + mat[i][n - i - 1];
        }

        if n % 2 == 1 {
            result -= mat[n / 2][n / 2];
        }

        return result;
    }
}

#[allow(dead_code)]
pub fn main() {
    let mat = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];

    assert_eq!(Solution::diagonal_sum(mat), 25);
}
