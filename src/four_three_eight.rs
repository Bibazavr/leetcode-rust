// https://leetcode.com/problems/find-all-anagrams-in-a-string/

struct Solution;

impl Solution {
    pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
        if s.len() < p.len() {
            return vec![];
        }

        let s = s.as_bytes();
        let mut count = vec![0; 26];
        for (c1, c2) in s.iter().zip(p.bytes()) {
            count[(c1 - b'a') as usize] += 1;
            count[(c2 - b'a') as usize] -= 1;
        }

        let mut ans = Vec::new();
        if count.iter().all(|&x| x == 0) {
            ans.push(0);
        }
        for i in p.len()..s.len() {
            count[(s[i] - b'a') as usize] += 1;
            count[(s[i - p.len()] - b'a') as usize] -= 1;
            if count.iter().all(|&x| x == 0) {
                ans.push((i - p.len() + 1) as i32);
            }
        }
        ans
    }
}

#[allow(dead_code)]
pub fn main() {
    let words = String::from("cbaebabacd");
    let order = String::from("abc");
    assert_eq!(Solution::find_anagrams(words, order), vec![0, 6]);

    let words = String::from("aaba");
    let order = String::from("aa");
    assert_eq!(Solution::find_anagrams(words, order), vec![0]);
}
