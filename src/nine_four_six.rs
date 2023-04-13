// https://leetcode.com/problems/validate-stack-sequences/

struct Solution;

impl Solution {
    pub fn validate_stack_sequences(pushed: Vec<i32>, popped: Vec<i32>) -> bool {
        let mut pushed_iter = pushed.iter();
        let mut stack = Vec::new();
        for &poped_next in popped.iter() {
            while stack.last() != Some(&poped_next) {
                match pushed_iter.next() {
                    Some(&x) => stack.push(x),
                    None => return false,
                };
            }
            stack.pop();
        }
        stack.is_empty()
    }
}

#[allow(dead_code)]
pub fn main() {
    let pushed = vec![2, 1, 0];
    let popped = vec![1, 2, 0];
    assert_eq!(Solution::validate_stack_sequences(pushed, popped), true);

    let pushed = vec![1, 2, 3, 4, 5];
    let popped = vec![4, 5, 3, 2, 1];
    assert_eq!(Solution::validate_stack_sequences(pushed, popped), true);
}
