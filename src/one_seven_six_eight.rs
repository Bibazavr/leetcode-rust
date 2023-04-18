// https://leetcode.com/problems/merge-strings-alternately/

struct Solution;

impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let mut result = String::with_capacity(word1.len() + word2.len());

        let mut chars1 = word1.chars();
        let mut chars2 = word2.chars();

        while let Some(c) = chars1.next() {
            result.push(c);
            if let Some(c2) = chars2.next() {
                result.push(c2);
            }
        }

        while let Some(c) = chars2.next() {
            result.push(c);
        }

        return result;
    }
}

#[allow(dead_code)]
pub fn main() {
    let word1 = String::from("abc");
    let word2 = String::from("pqr");
    assert_eq!(
        Solution::merge_alternately(word1, word2),
        String::from("apbqcr")
    );
}
