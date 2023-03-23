// https://leetcode.com/problems/number-of-zero-filled-subarrays/
struct Solution;

impl Solution {
    pub fn zero_filled_subarray(nums: Vec<i32>) -> i64 {
        let n = nums.len();

        let mut count = 0;

        let sum_of_series = |n: i64| n * (2 + (n - 1)) / 2;

        let mut i = 0;

        while i < n {
            while i < n && nums[i] != 0 {
                i += 1;
            }
            let j = i;
            while i < n && nums[i] == 0 {
                i += 1;
            }

            count += sum_of_series((i - j) as i64);
        }

        count
    }
}

#[allow(dead_code)]
pub fn main() {
    let nums = vec![1, 3, 0, 0, 2, 0, 0, 4];
    assert_eq!(Solution::zero_filled_subarray(nums), 6);
}
