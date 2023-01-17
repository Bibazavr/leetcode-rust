// https://leetcode.com/problems/flip-string-to-monotone-increasing/

struct Solution;

impl Solution {
    pub fn min_flips_mono_incr(s: String) -> i32 {
        let mut count0 = 0;
        let mut count1 = 0;
        for x in s.chars() {
            if x == '0' {
                count0 += 1;
            } else {
                count1 += 1;
            }
            count0 = count0.min(count1);
        }
        count0
    }
}

#[allow(dead_code)]
pub fn main() {
    let s = String::from("00110");
    assert_eq!(Solution::min_flips_mono_incr(s), 1);
}
