// https://leetcode.com/problems/count-ways-to-build-good-strings/

struct Solution;

impl Solution {
    pub fn count_good_strings(low: i32, high: i32, zero: i32, one: i32) -> i32 {
        use std::cmp::min;
        let modulus = 1_000_000_007;
        let mut dp = vec![0; (high + 2) as usize];

        for i in (0..=high).rev() {
            let mut cnt = if i >= low { 1 } else { 0 };

            let pick_zero = min(high + 1, i + zero);
            let pick_one = min(high + 1, i + one);

            cnt += (dp[pick_zero as usize] + dp[pick_one as usize]) % modulus;

            dp[i as usize] = cnt;
        }

        dp[0]
    }
}
#[allow(dead_code)]
pub fn main() {
    let low = 3;
    let high = 3;
    let zero = 1;
    let one = 1;
    assert_eq!(Solution::count_good_strings(low, high, zero, one), 8);
}
