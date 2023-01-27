// https://leetcode.com/problems/concatenated-words/

use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn find_all_concatenated_words_in_a_dict(words: Vec<String>) -> Vec<String> {
        let mut result = vec![];

        let mut hash = HashSet::new();

        words.iter().for_each(|word| {
            hash.insert(word.as_str());
        });

        let mut dp = vec![false; 31];
        dp[0] = true;
        for word in words.iter() {
            let n = word.len();

            if n == 1 {
                continue;
            }

            for i in 1..=n {
                for j in 0..i {
                    if j == 0 && i == n {
                        continue;
                    }

                    dp[i] = dp[j] && hash.contains(&word[j..i]);
                    if dp[i] {
                        break;
                    }
                }
            }
            if dp[n] {
                result.push(String::from(word));
            }
        }

        result
    }
}

#[allow(dead_code)]
pub fn main() {
    let words = vec![
        String::from("cat"),
        String::from("cats"),
        String::from("catsdogcats"),
        String::from("dog"),
        String::from("dogcatsdog"),
        String::from("hippopotamuses"),
        String::from("rat"),
        String::from("ratcatdogcat"),
    ];
    assert_eq!(
        Solution::find_all_concatenated_words_in_a_dict(words),
        vec!["catsdogcats", "dogcatsdog", "ratcatdogcat"],
    );
}
