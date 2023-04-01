// https://leetcode.com/problems/binary-search/

struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        match nums.binary_search(&target) {
            Ok(num) => num as i32,
            _ => -1,
        }
    }
}

#[allow(dead_code)]
pub fn main() {
    let s = vec![-1, 0, 3, 5, 9, 12];
    let num_rows = 9;
    assert_eq!(Solution::search(s, num_rows), 4);
}
