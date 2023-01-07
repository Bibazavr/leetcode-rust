use std::cmp::{max, min};

// https://leetcode.com/problems/median-of-two-sorted-arrays/
struct Solution;

// Bruteforce
// impl Solution {
//     pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
//         let mut all = [nums1, nums2].concat();
//         all.sort();
//
//         return if all.len() % 2 != 0 {
//             all[all.len() / 2] as f64
//         } else {
//             (all[all.len() / 2 - 1] + all[all.len() / 2]) as f64 / 2.0
//         };
//     }
// }

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let nums1_len = nums1.len();
        let nums2_len = nums2.len();

        if nums1_len > nums2_len {
            return Solution::find_median_sorted_arrays(nums2, nums1);
        }

        let mut start = 0;
        let mut end = nums1_len;

        while start <= end {
            let c1 = (start + end) / 2;
            let c2 = (nums1_len + nums2_len) / 2 - c1;

            let l1 = if c1 == 0 { i32::MIN } else { nums1[c1 - 1] };
            let l2 = if c2 == 0 { i32::MIN } else { nums2[c2 - 1] };

            let r1: i32 = if c1 == nums1_len { i32::MAX } else { nums1[c1] };
            let r2 = if c2 == nums2_len { i32::MAX } else { nums2[c2] };

            if l1 > r2 {
                end = c1 - 1
            } else if l2 > r1 {
                start = c1 + 1
            } else {
                return if (nums1_len + nums2_len) % 2 != 0 {
                    min(r1, r2) as f64
                } else {
                    (max(l1, l2) + min(r1, r2)) as f64 / 2.0
                };
            }
        }
        0.0
    }
}

#[allow(dead_code)]
pub fn main() {
    let nums1 = vec![1, 3];
    let nums2 = vec![2];

    assert_eq!(Solution::find_median_sorted_arrays(nums1, nums2), 2.00000);

    let nums1 = vec![1, 2];
    let nums2 = vec![3, 4];

    assert_eq!(Solution::find_median_sorted_arrays(nums1, nums2), 2.50000);

    let nums1 = vec![1, 2];
    let nums2 = vec![];

    assert_eq!(Solution::find_median_sorted_arrays(nums1, nums2), 1.50000);

    let nums1 = vec![];
    let nums2 = vec![2, 3];

    assert_eq!(Solution::find_median_sorted_arrays(nums1, nums2), 2.50000);

    let nums1 = vec![];
    let nums2 = vec![2, 3];

    assert_eq!(Solution::find_median_sorted_arrays(nums1, nums2), 2.50000);
}
