// https://leetcode.com/problems/minimize-maximum-of-array/

struct Solution;

impl Solution {
    pub fn minimize_array_value(nums: Vec<i32>) -> i32 {
        let mut sum: usize = 0;
        nums.into_iter()
            .enumerate()
            .map(|(i, n)| {
                sum += n as usize;
                (sum + i) / (i + 1)
            })
            .max()
            .unwrap_or(0) as i32
    }
}

#[allow(dead_code)]
pub fn main() {
    let nums = vec![3, 7, 1, 6];
    assert_eq!(Solution::minimize_array_value(nums), 5);
}
