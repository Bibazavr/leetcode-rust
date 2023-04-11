// https://leetcode.com/problems/valid-parentheses/
struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack: Vec<char> = Vec::with_capacity(s.len());

        for ch in s.chars() {
            match ch {
                '(' => stack.push(ch),
                '[' => stack.push(ch),
                '{' => stack.push(ch),
                _ => match stack.pop() {
                    None => {
                        return false;
                    }
                    Some(val) => match val {
                        '(' => {
                            if ch != ')' {
                                return false;
                            }
                        }
                        '[' => {
                            if ch != ']' {
                                return false;
                            }
                        }
                        '{' => {
                            if ch != '}' {
                                return false;
                            }
                        }
                        _ => {}
                    },
                },
            }
        }

        return stack.len() == 0;
    }
}

#[allow(dead_code)]
pub fn main() {
    let s = String::from("()");
    assert_eq!(Solution::is_valid(s), true);

    let s = String::from("()[]{}");
    assert_eq!(Solution::is_valid(s), true);

    let s = String::from("(]");
    assert_eq!(Solution::is_valid(s), false);
}
