// https://leetcode.com/problems/maximum-number-of-vowels-in-a-substring-of-given-length/

struct Solution;

impl Solution {
    fn is_vowel(ch: u8) -> bool {
        match ch {
            105 | 111 | 97 | 117 | 101 => true,
            _ => false,
        }
    }
    pub fn max_vowels(s: String, k: i32) -> i32 {
        let s = s.as_bytes();
        let k = k as usize;
        let count: usize = s.iter().take(k).filter(|&x| Self::is_vowel(*x)).count();
        s.iter()
            .skip(k)
            .zip(s.iter())
            .fold((count, count), |a, x| {
                let (max, count) = a;
                let (cur, left) = x;
                match (Self::is_vowel(*left), Self::is_vowel(*cur)) {
                    (false, true) => (max.max(count + 1), count + 1),
                    (true, false) => (max, count - 1),
                    _ => (max, count),
                }
            })
            .0 as i32
    }
}

#[allow(dead_code)]
pub fn main() {
    let s = String::from("abciiidef");
    let k = 3;

    assert_eq!(Solution::max_vowels(s, k), 3);
}
