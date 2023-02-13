// https://leetcode.com/problems/count-odd-numbers-in-an-interval-range/

struct Solution;

impl Solution {
    pub fn count_odds(low: i32, high: i32) -> i32 {
        (low..=high).fold(0, |mut acc, num| {
            if num % 2 != 0 {
                acc += 1;
            }
            acc
        })
    }
}

#[allow(dead_code)]
pub fn main() {
    let low = 3;

    let high = 7;

    assert_eq!(Solution::count_odds(low, high), 3);
}
