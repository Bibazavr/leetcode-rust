// https://leetcode.com/problems/count-subarrays-with-fixed-bounds/

struct Solution;

impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, min_k: i32, max_k: i32) -> i64 {
        let mut count = 0;
        let mut last_out: i64 = -1;
        let mut last_min: i64 = -1;
        let mut last_max: i64 = -1;

        for (i, num) in nums.into_iter().enumerate() {
            if min_k <= num && num <= max_k {
                if num == min_k {
                    last_min = i as i64;
                }
                if num == max_k {
                    last_max = i as i64;
                }

                count += 0i64.max(last_min.min(last_max) - last_out);
            } else {
                last_out = i as i64;
            }
        }

        count
    }
}

#[allow(dead_code)]
pub fn main() {
    let nums = vec![1, 1, 1, 1];
    let min_k = 1;
    let max_k = 1;
    assert_eq!(Solution::count_subarrays(nums, min_k, max_k), 10);

    let nums = vec![1, 3, 5, 2, 7, 5];
    let min_k = 1;
    let max_k = 5;
    assert_eq!(Solution::count_subarrays(nums, min_k, max_k), 2);
}
