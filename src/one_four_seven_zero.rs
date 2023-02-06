// https://leetcode.com/problems/shuffle-the-array/

struct Solution;

impl Solution {
    pub fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
        let mut result = vec![];

        let (left_arr, right_arr) = nums.split_at(n as usize);

        // More elegant way (Leetcode have old version of rust)
        // for (&left, &right) in std::iter::zip(left_arr, right_arr) {
        //     result.push(left);
        //     result.push(right);
        // }

        for (&left, &right) in left_arr.iter().zip(right_arr) {
            result.push(left);
            result.push(right);
        }

        result
    }
}

#[allow(dead_code)]
pub fn main() {
    let nums = vec![2, 5, 1, 3, 4, 7];
    let n = 3;
    assert_eq!(Solution::shuffle(nums, n), vec![2, 3, 5, 4, 1, 7]);
}
