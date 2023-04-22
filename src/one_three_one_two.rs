// https://leetcode.com/problems/minimum-insertion-steps-to-make-a-string-palindrome/

struct Solution;

impl Solution {
    pub fn min_insertions(s: String) -> i32 {
        let bytes_arr = s.into_bytes();
        let n = bytes_arr.len();

        let mut prev = vec![0; n];

        for i in (0..n - 1).rev() {
            let mut curr = vec![0; n];
            for j in i + 1..n {
                if bytes_arr[i] == bytes_arr[j] {
                    curr[j] = prev[j - 1];
                } else {
                    curr[j] = 1 + prev[j].min(curr[j - 1]);
                }
            }
            prev = curr;
        }

        return prev[n - 1];
    }
}

#[allow(dead_code)]
pub fn main() {
    let s = String::from("zzazz");
    assert_eq!(Solution::min_insertions(s), 0);

    let s = String::from("mbadm");
    assert_eq!(Solution::min_insertions(s), 2);
}
