// https://leetcode.com/problems/sign-of-the-product-of-an-array/

struct Solution;

impl Solution {
    pub fn array_sign(nums: Vec<i32>) -> i32 {
        nums.iter().map(|x| x.signum()).product()
    }
}

#[allow(dead_code)]
pub fn main() {
    let nums = vec![-1, -2, -3, -4, 3, 2, 1];
    assert_eq!(Solution::array_sign(nums), 1)
}
