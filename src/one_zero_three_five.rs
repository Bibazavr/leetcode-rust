// https://leetcode.com/problems/uncrossed-lines/

struct Solution;

impl Solution {
    pub fn max_uncrossed_lines(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        use std::cmp::max;

        // Vector to store a running maximum number of lines
        let mut dp = vec![0; nums2.len() + 1];

        for n1 in nums1 {
            let mut prev = 0;
            for (j, &n2) in nums2.iter().enumerate() {
                let curr = dp[j + 1];
                if n1 == n2 {
                    dp[j + 1] = prev + 1;
                } else {
                    dp[j + 1] = max(dp[j + 1], dp[j]);
                }
                prev = curr;
            }
        }
        dp[nums2.len()]
    }
}

#[allow(dead_code)]
pub fn main() {
    let nums1 = vec![1, 4, 2];
    let nums2 = vec![1, 2, 4];
    assert_eq!(Solution::max_uncrossed_lines(nums1, nums2), 2);
}
