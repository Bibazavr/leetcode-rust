// https://leetcode.com/problems/minimize-deviation-in-array/

struct Solution;

impl Solution {
    pub fn minimum_deviation(nums: Vec<i32>) -> i32 {
        let mut low = i32::MAX;
        let mut heap = std::collections::BinaryHeap::with_capacity(nums.len());
        for mut num in nums.into_iter() {
            if num & 1 == 1 {
                num <<= 1;
            }
            low = low.min(num);
            heap.push(num);
        }
        let mut result = i32::MAX;

        while let Some(high) = heap.pop() {
            result = result.min(high - low);
            if high & 1 == 1 {
                break;
            }
            low = low.min(high >> 1);
            heap.push(high >> 1);
        }

        result
    }
}

#[allow(dead_code)]
pub fn main() {
    let nums = vec![1, 2, 3, 4];
    assert_eq!(Solution::minimum_deviation(nums), 1);
}
