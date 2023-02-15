// https://leetcode.com/problems/add-to-array-form-of-integer/

struct Solution;

impl Solution {
    pub fn add_to_array_form(num: Vec<i32>, k: i32) -> Vec<i32> {
        let mut result = num.clone();
        let mut carry = k;
        for i in (0..result.len()).rev() {
            result[i] += carry;
            carry = result[i] / 10;
            result[i] %= 10;
        }

        while carry > 0 {
            result.insert(0, carry % 10);
            carry /= 10;
        }
        result
    }
}

#[allow(dead_code)]
pub fn main() {
    let num = vec![1, 2, 0, 0];
    let k = 34;
    assert_eq!(Solution::add_to_array_form(num, k), vec![1, 2, 3, 4]);

    let num = vec![2, 7, 4];
    let k = 181;
    assert_eq!(Solution::add_to_array_form(num, k), vec![4, 5, 5]);
}
