// https://leetcode.com/problems/minimum-cost-for-tickets/

struct Solution;

impl Solution {
    pub fn mincost_tickets(days: Vec<i32>, costs: Vec<i32>) -> i32 {
        use std::cmp::{max, min};
        let last_day = days[days.len() - 1] as usize;
        let mut table = vec![0; last_day + 1];
        let mut counter = 0usize;
        for i in 1..last_day + 1 {
            println!("{:?}", table);
            if i as i32 == days[counter] {
                let mut value = 1_000_000;
                for (cost, period) in costs.iter().zip(vec![1, 7, 30]) {
                    let normal_i = i as i32;
                    let minimal_index = max(0, normal_i - period) as usize;
                    value = min(table[minimal_index] + cost, value);
                }
                table[i] = value;
                counter += 1;
            } else {
                table[i] = table[i - 1];
            }
        }

        table[last_day]
    }
}

#[allow(dead_code)]
pub fn main() {
    let days = vec![1, 4, 6, 7, 8, 20];
    let costs = vec![2, 7, 15];
    assert_eq!(Solution::mincost_tickets(days, costs), 11);
}
