// https://leetcode.com/problems/last-stone-weight/

struct Solution;

impl Solution {
    pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
        let mut stones: std::collections::BinaryHeap<_> = stones.into();
        while !stones.is_empty() {
            match (stones.pop(), stones.pop()) {
                (Some(a), Some(b)) if a > b => stones.push(a - b),
                (Some(a), None) => return a,
                _ => (),
            }
        }
        0
    }
}

#[allow(dead_code)]
pub fn main() {
    let stones = vec![2, 7, 4, 1, 8, 1];
    assert_eq!(Solution::last_stone_weight(stones), 1);
}
