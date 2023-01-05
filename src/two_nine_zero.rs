use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn word_pattern(pattern: String, s: String) -> bool {
        let mut map_s2i: HashMap<&str, usize> = HashMap::new();
        let mut map_c2i: HashMap<char, usize> = HashMap::new();
        let arr_patterns: Vec<_> = pattern.chars().collect();
        let arr_strings: Vec<_> = s.split(" ").collect();

        if arr_strings.len() != arr_patterns.len() {
            return false;
        }

        for (i, str) in arr_strings.into_iter().enumerate() {
            let c = arr_patterns[i];

            match (map_s2i.get(str), map_c2i.get(&c)) {
                (Some(si), Some(ci)) => {
                    if si != ci {
                        return false;
                    }
                }
                (Some(_si), None) => return false,
                (None, Some(_ci)) => return false,
                (None, None) => {
                    map_s2i.insert(str, i);
                    map_c2i.insert(c, i);
                }
            }
        }
        true
    }
}

#[allow(dead_code)]
pub fn main() {
    let pattern = "abba".to_string();
    let string = "dog cat cat fish".to_string();
    let answer = Solution::word_pattern(pattern, string);
    assert_eq!(answer, false);

    let pattern = "abba".to_string();
    let string = "dog cat cat dog".to_string();
    let answer = Solution::word_pattern(pattern, string);
    assert_eq!(answer, true);

    let pattern = "abba".to_string();
    let string = "dog dog dog dog".to_string();
    let answer = Solution::word_pattern(pattern, string);
    assert_eq!(answer, false);

    let pattern = "aba".to_string();
    let string = "dog cat cat".to_string();
    let answer = Solution::word_pattern(pattern, string);
    assert_eq!(answer, false);

    let pattern = "aaa".to_string();
    let string = "aa aa aa aa".to_string();
    let answer = Solution::word_pattern(pattern, string);
    assert_eq!(answer, false);
}
