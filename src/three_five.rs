// https://leetcode.com/problems/search-insert-position/

struct Solution;

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let (mut l, mut r) = (0, nums.len());
        while l < r {
            let mid = l + ((r - l) >> 1);
            if nums[mid] < target {
                l = mid + 1;
            } else {
                r = mid;
            }
        }
        l as _
    }
}

#[allow(dead_code)]
pub fn main() {
    let nums = vec![1, 3, 5, 6];
    let target = 5;
    assert_eq!(Solution::search_insert(nums, target), 2);
}
