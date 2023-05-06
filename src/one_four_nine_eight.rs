// https://leetcode.com/problems/number-of-subsequences-that-satisfy-the-given-sum-condition/

struct Solution;

impl Solution {
    pub fn num_subseq(mut nums: Vec<i32>, target: i32) -> i32 {
        nums.sort_unstable();

        let m = 1_000_000_007;
        let mut result = 0;

        let fast_pow = |mut n: usize| -> usize {
            let mut res = 1;
            let mut base = 2;
            while n > 0 {
                if n & 1 == 1 {
                    res = res * base % m;
                }
                n >>= 1;
                base = base * base % m;
            }
            res
        };

        for i in 0..nums.len() {
            if nums[i] > target / 2 {
                break;
            }

            let target = target - nums[i];
            let mut l = i;
            let mut r = nums.len();
            while l < r {
                let mid = l + (r - l) / 2;
                if nums[mid] <= target {
                    l = mid + 1;
                } else {
                    r = mid;
                }
            }
            result = (result + fast_pow(l - i - 1)) % m;
        }
        result as i32
    }
}

#[allow(dead_code)]
pub fn main() {
    let nums = vec![2, 3, 3, 4, 6, 7];
    let target = 12;
    assert_eq!(Solution::num_subseq(nums, target), 61);

    let nums = vec![3, 3, 6, 8];
    let target = 10;
    assert_eq!(Solution::num_subseq(nums, target), 6);

    let nums = vec![3, 5, 6, 7];
    let target = 9;
    assert_eq!(Solution::num_subseq(nums, target), 4);
}
