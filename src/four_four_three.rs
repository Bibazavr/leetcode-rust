// https://leetcode.com/problems/string-compression/
struct Solution;

impl Solution {
    pub fn compress(chars: &mut Vec<char>) -> i32 {
        let (mut next, mut left, n) = (0, 0, chars.len());
        for right in 1..=n {
            if right == n || chars[right] != chars[left] {
                chars[next] = chars[left];
                next += 1;
                if right - left > 1 {
                    for c in (right - left).to_string().chars() {
                        chars[next] = c;
                        next += 1;
                    }
                }
                left = right;
            }
        }
        next as _
    }
}

#[allow(dead_code)]
pub fn main() {
    let mut chars = vec!['a', 'a', 'b', 'b', 'c', 'c', 'c'];
    assert_eq!(Solution::compress(&mut chars), 6);
}
