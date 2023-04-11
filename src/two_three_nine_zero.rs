// https://leetcode.com/problems/removing-stars-from-a-string/
struct Solution;

impl Solution {
    pub fn remove_stars(s: String) -> String {
        s.chars().fold(String::new(), |mut result, c| {
            match c {
                '*' => {
                    result.pop();
                }
                _ => result.push(c),
            };
            result
        })
    }
}

#[allow(dead_code)]
pub fn main() {
    let ideas = String::from("leet**cod*e");
    assert_eq!(Solution::remove_stars(ideas), String::from("lecoe"));
}
