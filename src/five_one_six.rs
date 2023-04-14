// https://leetcode.com/problems/longest-palindromic-subsequence/

struct Solution;

impl Solution {
    pub fn longest_palindrome_subseq(s: String) -> i32 {
        let bytes: &[u8] = s.as_bytes();
        let n = bytes.len();

        if n == 1 {
            return 1;
        }

        let mut cache: Vec<Vec<i32>> = vec![vec![0; n]; n];

        for i in (0..n).rev() {
            cache[i][i] = 1;

            for j in i + 1..n {
                cache[i][j] = match bytes[i] == bytes[j] {
                    true => 2 + cache[i + 1][j - 1],
                    false => std::cmp::max(cache[i + 1][j], cache[i][j - 1]),
                };
            }
        }

        cache[0][n - 1]
    }
}

#[allow(dead_code)]
pub fn main() {
    let s = String::from("bbbab");
    assert_eq!(Solution::longest_palindrome_subseq(s), 4);
}
