// https://leetcode.com/problems/restore-the-array/

struct Solution;

impl Solution {
    pub fn number_of_arrays(s: String, k: i32) -> i32 {
        let n = s.len();
        let s = s.as_bytes();
        let k = k as u64;
        let m = 1_000_000_007;

        let mut dp = vec![0; n + 1];
        dp[n] = 1;

        for i in (0..n).rev() {
            if s[i] != b'0' {
                let mut cur = 0;
                for j in i..n {
                    // when k is large enough, cur * 10 may overflow for i32
                    cur = cur * 10 + (s[j] - b'0') as u64;
                    if cur > k {
                        break;
                    }

                    dp[i] = (dp[i] + dp[j + 1]) % m;
                }
            }
        }

        dp[0]
    }
}

#[allow(dead_code)]
pub fn main() {
    let s = String::from("1000");
    let k = 1000;
    assert_eq!(Solution::number_of_arrays(s, k), 1);
}
