// https://leetcode.com/problems/sort-an-array/

struct Solution;

impl Solution {
    pub fn sort_array(nums: Vec<i32>) -> Vec<i32> {
        Self::heap_sort(nums)
    }

    pub fn heapify(nums: &mut Vec<i32>, n: usize, i: usize) {
        let mut largest = i;
        let l = 2 * i + 1;
        let r = 2 * i + 2;

        if l < n && nums[l] > nums[largest] {
            largest = l
        }

        if r < n && nums[r] > nums[largest] {
            largest = r
        }

        if largest != i {
            nums.swap(i, largest);

            Self::heapify(nums, n, largest);
        }
    }
    pub fn heap_sort(mut nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();

        for i in (0..n / 2).rev() {
            Self::heapify(&mut nums, n, i)
        }

        for i in (1..n).rev() {
            nums.swap(0, i);

            Self::heapify(&mut nums, i, 0)
        }

        nums
    }
}

#[allow(dead_code)]
pub fn main() {
    let nums = vec![5, 2, 3, 1];
    assert_eq!(Solution::sort_array(nums), vec![1, 2, 3, 5]);
}
