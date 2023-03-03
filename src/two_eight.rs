//https://leetcode.com/problems/find-the-index-of-the-first-occurrence-in-a-string/

struct Solution;

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        match haystack.find(&needle) {
            None => -1,
            Some(i) => i as i32,
        }
    }
}

#[allow(dead_code)]
pub fn main() {
    let haystack = String::from("sadbutsad");
    let needle = String::from("sad");
    assert_eq!(Solution::str_str(haystack, needle), 0);
}
