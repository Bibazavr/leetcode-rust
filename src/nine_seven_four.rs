// https://leetcode.com/problems/subarray-sums-divisible-by-k/

struct Solution;

impl Solution {
    pub fn subarrays_div_by_k(nums: Vec<i32>, k: i32) -> i32 {
        let mut answer = 0;
        let mut prefix_sum = 0;
        let mut rem = vec![0; k as usize];
        rem[0] = 1;
        nums.iter().for_each(|num| {
            prefix_sum += num;
            let key = (prefix_sum % k + k) % k;
            answer += rem[key as usize];
            rem[key as usize] += 1
        });
        answer
    }
}

#[allow(dead_code)]
pub fn main() {
    let nums = vec![4, 5, 0, -2, -3, 1];
    let k = 5;
    assert_eq!(Solution::subarrays_div_by_k(nums, k), 7);

    let nums = vec![5];
    let k = 9;
    assert_eq!(Solution::subarrays_div_by_k(nums, k), 0);

    let nums = vec![5, 10, 15];
    let k = 5;
    assert_eq!(Solution::subarrays_div_by_k(nums, k), 6);
}
