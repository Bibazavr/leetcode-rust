// https://leetcode.com/problems/optimal-partition-of-string/

struct Solution;

impl Solution {
    pub fn partition_string(s: String) -> i32 {
        let mut res = 1;
        let mut mask = 0;
        for &c in s.as_bytes() {
            let cur = 1 << (c - b'a');
            if (mask & cur) == cur {
                res += 1;
                mask = 0;
            }
            mask ^= cur;
        }
        res
    }
}

#[allow(dead_code)]
pub fn main() {
    let vals = String::from("abacaba");
    assert_eq!(Solution::partition_string(vals), 4);
}
