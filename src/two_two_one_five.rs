// https://leetcode.com/problems/find-the-difference-of-two-arrays/

use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn find_difference(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<Vec<i32>> {
        let set1: HashSet<i32> = nums1.into_iter().collect();
        let set2: HashSet<i32> = nums2.into_iter().collect();

        vec![
            set1.difference(&set2).map(|&x| x).collect(),
            set2.difference(&set1).map(|&x| x).collect(),
        ]
    }
}
#[allow(dead_code)]
pub fn main() {
    let nums1 = vec![1, 2, 3];
    let nums2 = vec![2, 4, 6];

    assert_eq!(
        Solution::find_difference(nums1, nums2),
        vec![vec![1, 3], vec![4, 6]]
    );
}
