// https://leetcode.com/problems/kth-missing-positive-number/

struct Solution;

impl Solution {
    pub fn find_kth_positive(arr: Vec<i32>, mut k: i32) -> i32 {
        let mut correct_num = 0;
        for num in arr {
            correct_num += 1;
            if num == correct_num {
                continue;
            }

            for missed_num in correct_num..num {
                k -= 1;
                if k == 0 {
                    return missed_num;
                } else {
                    correct_num += 1;
                }
            }
        }

        correct_num + k
    }
}

#[allow(dead_code)]
pub fn main() {
    let arr = vec![1, 2, 3, 4];
    let k = 2;

    assert_eq!(Solution::find_kth_positive(arr, k), 6);

    let arr = vec![2, 3, 4, 7, 11];
    let k = 5;

    assert_eq!(Solution::find_kth_positive(arr, k), 9);
}
