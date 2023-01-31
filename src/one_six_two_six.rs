// https://leetcode.com/problems/best-team-with-no-conflicts/

struct Solution;

impl Solution {
    pub fn best_team_score(scores: Vec<i32>, ages: Vec<i32>) -> i32 {
        let mut ans = 0;
        let n = scores.len();
        let mut players: Vec<Vec<i32>> = vec![vec![]; n];
        let mut dp: Vec<i32> = vec![0; n];

        for i in 0..n {
            players[i].push(ages[i]);
            players[i].push(scores[i]);
        }

        players.sort_by(|a, b| b.cmp(a));

        for i in 0..n {
            dp[i] = players[i][1];
            for j in 0..i {
                if players[j][1] >= players[i][1] {
                    dp[i] = dp[i].max(dp[j] + players[i][1]);
                }
            }
            ans = ans.max(dp[i]);
        }

        ans
    }
}

#[allow(dead_code)]
pub fn main() {
    let scores = vec![1, 3, 5, 10, 15];
    let ages = vec![1, 2, 3, 4, 5];
    assert_eq!(Solution::best_team_score(scores, ages), 34);
}
