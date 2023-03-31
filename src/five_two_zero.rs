// https://leetcode.com/problems/detect-capital/
struct Solution;

impl Solution {
    pub fn detect_capital_use(word: String) -> bool {
        word.chars().skip(1).all(|c| c.is_lowercase()) || word.chars().all(|c| c.is_uppercase())
    }
}

#[allow(dead_code)]
pub fn main() {
    let word = String::from("USA");
    assert_eq!(Solution::detect_capital_use(word), true);
}
