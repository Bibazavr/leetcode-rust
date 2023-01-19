// https://leetcode.com/problems/maximum-sum-circular-subarray/

struct Solution;

impl Solution {
    pub fn max_subarray_sum_circular(nums: Vec<i32>) -> i32 {
        use std::cmp::{max, min};

        let mut total = 0;
        let mut total_min = i32::MAX;
        let mut total_max = i32::MIN;
        let mut curr_max = 0;
        let mut curr_min = 0;
        nums.iter().for_each(|num| {
            total += num;
            curr_max += num;
            curr_min += num;
            total_max = max(curr_max, total_max);
            total_min = min(curr_min, total_min);

            if curr_max < 0 {
                curr_max = 0;
            }

            if curr_min > 0 {
                curr_min = 0
            }
        });

        return if total_max > 0 {
            max(total_max, total - total_min)
        } else {
            total_max
        };
    }
}

#[allow(dead_code)]
pub fn main() {
    let nums = vec![1, -2, 3, -2];
    assert_eq!(Solution::max_subarray_sum_circular(nums), 3);

    let nums = vec![5, -3, 5];
    assert_eq!(Solution::max_subarray_sum_circular(nums), 10);

    let nums = vec![-3, -2, -3];
    assert_eq!(Solution::max_subarray_sum_circular(nums), -2);
}
