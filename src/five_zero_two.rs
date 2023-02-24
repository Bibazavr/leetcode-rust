// https://leetcode.com/problems/ipo/
struct Solution;

impl Solution {
    pub fn find_maximized_capital(k: i32, mut w: i32, profits: Vec<i32>, capital: Vec<i32>) -> i32 {
        let n = profits.len();
        let mut capital_and_profits = Vec::with_capacity(n);

        for i in 0..n {
            capital_and_profits.push((capital[i], profits[i]));
        }
        capital_and_profits.sort();

        let mut priority_queue = std::collections::BinaryHeap::<i32>::with_capacity(n);
        let mut i = 0;

        for _ in 0..k {
            while i < n && capital_and_profits[i].0 <= w {
                priority_queue.push(capital_and_profits[i].1);
                i += 1;
            }

            w += if let Some(profit) = priority_queue.pop() {
                profit
            } else {
                break;
            };
        }
        return w;
    }
}
#[allow(dead_code)]
pub fn main() {
    let k = 10;
    let w = 0;
    let profits = vec![1, 2, 3];
    let capital = vec![0, 1, 2];
    assert_eq!(Solution::find_maximized_capital(k, w, profits, capital), 6);

    let k = 2;
    let w = 0;
    let profits = vec![1, 2, 3];
    let capital = vec![0, 1, 1];
    assert_eq!(Solution::find_maximized_capital(k, w, profits, capital), 4);
}
