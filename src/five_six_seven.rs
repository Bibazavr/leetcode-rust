// https://leetcode.com/problems/permutation-in-string/
struct Solution;

impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        if s1.len() > s2.len() {
            return false;
        }
        let s2 = s2.as_bytes();
        let mut count = vec![0; 26];

        for (c1, c2) in s1.bytes().zip(s2.iter()).take(s1.len()) {
            count[(c1 - b'a') as usize] += 1;
            count[(c2 - b'a') as usize] -= 1;
        }

        for i in s1.len()..s2.len() {
            if count.iter().all(|&x| x == 0) {
                return true;
            }

            count[(s2[i] - b'a') as usize] -= 1;
            count[(s2[i - s1.len()] - b'a') as usize] += 1;
        }
        count.iter().all(|&x| x == 0)
    }
}

#[allow(dead_code)]
pub fn main() {
    let nums1 = String::from("ab");
    let nums2 = String::from("eidbaooo");

    assert_eq!(Solution::check_inclusion(nums1, nums2), true);

    let nums1 = String::from("ab");
    let nums2 = String::from("eidboaoo");

    assert_eq!(Solution::check_inclusion(nums1, nums2), false);
}
