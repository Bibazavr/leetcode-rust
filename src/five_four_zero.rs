// https://leetcode.com/problems/single-element-in-a-sorted-array/
struct Solution;
impl Solution {
    pub fn single_non_duplicate(nums: Vec<i32>) -> i32 {
        let (mut l, mut r) = (0, nums.len() - 1);
        while l < r {
            let mid = l + ((r - l) >> 1);
            if nums[mid] == nums[mid ^ 1] {
                l = mid + 1;
            } else {
                r = mid;
            }
        }
        nums[l]
    }
}
#[allow(dead_code)]
pub fn main() {
    let nums = vec![1];

    assert_eq!(Solution::single_non_duplicate(nums), 1);

    let nums = vec![3, 3, 7, 7, 10, 11, 11];

    assert_eq!(Solution::single_non_duplicate(nums), 10);

    let nums = vec![1, 1, 2, 3, 3, 4, 4, 8, 8];

    assert_eq!(Solution::single_non_duplicate(nums), 2);
}
