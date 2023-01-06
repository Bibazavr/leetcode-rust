// https://leetcode.com/problems/longest-substring-without-repeating-characters/

struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut max = 0;
        let arr = s.chars().collect::<Vec<_>>();

        if arr.len() == 1 {
            return 1;
        }

        let last_len = arr
            .into_iter()
            .fold(String::from(""), |mut acc, item| {
                if acc.contains(item) {
                    let len = acc.len();
                    if len > max {
                        max = len
                    }

                    let item_index = acc.find(item).unwrap();

                    acc = acc.chars().skip(item_index + 1).collect()
                }
                acc.push(item);
                acc
            })
            .len();

        if last_len > max {
            return last_len as i32;
        } else {
            max as i32
        }
    }
}

#[allow(dead_code)]
pub fn main() {
    let s = String::from("abcabcbb");
    assert_eq!(Solution::length_of_longest_substring(s), 3);

    let s = String::from(" ");
    assert_eq!(Solution::length_of_longest_substring(s), 1);

    let s = String::from("");
    assert_eq!(Solution::length_of_longest_substring(s), 0);

    let s = String::from("au");
    assert_eq!(Solution::length_of_longest_substring(s), 2);

    let s = String::from("dvdf");
    assert_eq!(Solution::length_of_longest_substring(s), 3);
}
