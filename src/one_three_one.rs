// https://leetcode.com/problems/palindrome-partitioning/

struct Solution;

impl Solution {
    pub fn partition(s: String) -> Vec<Vec<String>> {
        let mut result: Vec<_> = vec![];
        let mut stack = vec![];

        Self::backtrack(0, &s, &mut result, &mut stack);

        result
    }

    fn backtrack(start: usize, s: &String, result: &mut Vec<Vec<String>>, stack: &mut Vec<String>) {
        if start >= s.len() {
            result.push(stack.clone());
            return;
        }

        for i in (start + 1)..=s.len() {
            if Self::is_palindrome(&s[start..i]) {
                stack.push(s[start..i].to_string());
                Self::backtrack(i, &s, result, stack);
                stack.pop();
            }
        }
    }

    pub fn is_palindrome(s: &str) -> bool {
        let mut left = 0;
        let mut right = s.len() - 1;
        while left < right {
            if &s[left..left + 1] != &s[right..right + 1] {
                return false;
            }
            left += 1;
            right -= 1;
        }
        return true;
    }
}

#[allow(dead_code)]
pub fn main() {
    let s = String::from("aab");
    assert_eq!(
        Solution::partition(s),
        vec![vec!["a", "a", "b"], vec!["aa", "b"]]
    )
}
