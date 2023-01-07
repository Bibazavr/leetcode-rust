// https://leetcode.com/problems/gas-station/description/

struct Solution;

impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let mut remain = 0;
        let mut curr = 0;
        let mut start = 0;

        for i in 0..gas.len() {
            curr += gas[i] - cost[i];
            if curr < 0 {
                start = i + 1;
                curr = 0;
            }
            remain += gas[i] - cost[i];
        }

        return if remain >= 0 { start as i32 } else { -1 };
    }
}

#[allow(dead_code)]
pub fn main() {
    let costs = vec![1, 2, 3, 4, 5];
    let coins = vec![3, 4, 5, 1, 2];
    assert_eq!(Solution::can_complete_circuit(costs, coins), 3)
}
