// https://leetcode.com/problems/best-time-to-buy-and-sell-stock/

struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let (mut profit, mut buy) = (0, prices[0]);

        for i in 1..prices.len() {
            profit = i32::max(profit, prices[i] - buy);
            buy = i32::min(buy, prices[i]);
        }

        return profit;
    }
}

#[allow(dead_code)]
pub fn main() {
    let prices = vec![7, 1, 5, 3, 6, 4];
    assert_eq!(Solution::max_profit(prices), 5);
}
