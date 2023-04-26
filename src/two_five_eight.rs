// https://leetcode.com/problems/add-digits/

struct Solution;

impl Solution {
    pub fn add_digits(mut num: i32) -> i32 {
        return match num {
            0 => 0,
            _ => {
                num = num % 9;
                if num == 0 {
                    9
                } else {
                    num
                }
            }
        };
    }
}

#[allow(dead_code)]
pub fn main() {
    let num = 38;
    assert_eq!(Solution::add_digits(num), 2);
}
