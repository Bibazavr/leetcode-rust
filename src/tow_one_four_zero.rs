// https://leetcode.com/problems/solving-questions-with-brainpower/

struct Solution;

impl Solution {
    pub fn most_points(questions: Vec<Vec<i32>>) -> i64 {
        use std::cmp::{max, min};
        let n = questions.len();
        let mut dp: Vec<i64> = vec![0; n + 1];

        for i in (0..n).rev() {
            let not_pick = dp[i + 1];
            let pick = dp[min(n, i + questions[i][1] as usize + 1)] + questions[i][0] as i64;

            dp[i] = max(pick, not_pick);
        }

        dp[0]
    }
}

#[allow(dead_code)]
pub fn main() {
    let questions = vec![vec![3, 2], vec![4, 3], vec![4, 4], vec![2, 5]];
    assert_eq!(Solution::most_points(questions), 5);
}
