// https://leetcode.com/problems/profitable-schemes/
struct Solution;

impl Solution {
    pub fn profitable_schemes(n: i32, min_profit: i32, group: Vec<i32>, profit: Vec<i32>) -> i32 {
        let n = n as usize;
        let min_profit = min_profit as usize;
        let mut dp = vec![vec![0; min_profit + 1]; n + 1];
        let m = 1_000_000_007;

        dp[0][0] = 1;

        for i in 0..group.len() {
            let g = group[i] as usize;
            let p = profit[i] as usize;

            if n < g {
                continue;
            }

            for member in (0..=(n - g)).rev() {
                for profit in 0..=min_profit {
                    dp[member + g][min_profit.min(profit + p)] =
                        (dp[member][profit] + dp[member + g][min_profit.min(profit + p)]) % m;
                }
            }
        }

        let mut result = 0;
        for i in 0..=n {
            result = (result + dp[i][min_profit]) % m;
        }
        result
    }
}

#[allow(dead_code)]
pub fn main() {
    let n = 5;
    let min_profit = 3;
    let group = vec![2, 2];
    let piles = vec![2, 3];

    assert_eq!(Solution::profitable_schemes(n, min_profit, group, piles), 2);
}
