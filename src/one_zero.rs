// https://leetcode.com/problems/regular-expression-matching/description/

struct Solution;

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        return Solution::is_match_slice(s.as_bytes(), p.as_bytes());
    }

    fn is_match_slice(s: &[u8], p: &[u8]) -> bool {
        return match (s, p) {
            ([first_s, rest_s @ ..], [first_p, b'*', rest_p @ ..]) => {
                return if first_s == first_p || first_p == &b'.' {
                    Solution::is_match_slice(rest_s, p) || Solution::is_match_slice(s, rest_p)
                } else {
                    Solution::is_match_slice(s, rest_p)
                }
            }
            ([first_s, rest_s @ ..], [first_p, rest_p @ ..]) => {
                return if first_s == first_p || first_p == &b'.' {
                    Solution::is_match_slice(rest_s, rest_p)
                } else {
                    false
                }
            }
            ([], [_, b'*', rest_p @ ..]) => Solution::is_match_slice(s, rest_p),
            ([], []) => true,
            (_, _) => false,
        };
    }
}

#[allow(dead_code)]
pub fn main() {
    let s = String::from("aa");
    let p = String::from("");
    assert_eq!(Solution::is_match(s, p), false);

    let s = String::from("aa");
    let p = String::from("a*");
    assert_eq!(Solution::is_match(s, p), true);

    let s = String::from("aa");
    let p = String::from(".*");
    assert_eq!(Solution::is_match(s, p), true);

    let s = String::from("aaa");
    let p = String::from("a*a");
    assert_eq!(Solution::is_match(s, p), true);

    let s = String::from("ab");
    let p = String::from(".*c");
    assert_eq!(Solution::is_match(s, p), false);
}
