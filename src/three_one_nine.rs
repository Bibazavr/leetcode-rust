// https://leetcode.com/problems/bulb-switcher/

struct Solution;

impl Solution {
    pub fn bulb_switch(n: i32) -> i32 {
        return f64::sqrt(n as f64) as i32;
    }
}

#[allow(dead_code)]
pub fn main() {
    let n = 3;
    assert_eq!(Solution::bulb_switch(n), 1);
}
