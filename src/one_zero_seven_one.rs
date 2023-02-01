// https://leetcode.com/problems/greatest-common-divisor-of-strings/

struct Solution;

impl Solution {
    pub fn gcd_of_strings(str1: String, str2: String) -> String {
        let mut result = String::new();

        let chars1 = str1.as_bytes();
        let chars2 = str2.as_bytes();

        for len in 1..=str1.len().min(str1.len()) {
            let substring = &chars1[..len];

            if str1.len() % substring.len() != 0 || str2.len() % substring.len() != 0 {
                continue;
            }
            if !chars1
                .chunks(substring.len())
                .all(|chunk| chunk == substring)
            {
                continue;
            }
            if !chars2
                .chunks(substring.len())
                .all(|chunk| chunk == substring)
            {
                continue;
            }

            result = String::from_utf8_lossy(substring).parse().unwrap()
        }
        result
    }
}

#[allow(dead_code)]
pub fn main() {
    let str1 = String::from("ABCABC");
    let str2 = String::from("ABC");
    assert_eq!(Solution::gcd_of_strings(str1, str2), String::from("ABC"));
}
