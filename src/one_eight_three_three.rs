// https://leetcode.com/problems/maximum-ice-cream-bars/

struct Solution;

impl Solution {
    pub fn max_ice_cream(mut costs: Vec<i32>, mut coins: i32) -> i32 {
        let mut count = 0;
        costs.sort_unstable();
        for cost in costs.into_iter() {
            coins = coins - cost;
            if coins >= 0 {
                count += 1;
            } else {
                break;
            }
        }
        count
    }
}

#[allow(dead_code)]
pub fn main() {
    let costs = vec![1, 3, 2, 4, 1];
    let coins = 7;
    assert_eq!(Solution::max_ice_cream(costs, coins), 4)
}
