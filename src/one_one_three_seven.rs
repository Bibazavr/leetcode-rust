// https://leetcode.com/problems/n-th-tribonacci-number/

struct Solution;

impl Solution {
    pub fn tribonacci(n: i32) -> i32 {
        let mut arr = Vec::new();
        arr.push(0);
        arr.push(1);
        arr.push(1);

        if n < 3 {
            return arr[n as usize];
        } else {
            for i in 0..=n {
                arr.push(arr[i as usize] + arr[i as usize + 1] + arr[i as usize + 2]);
            }
        }

        return arr[n as usize];
    }
}

#[allow(dead_code)]
pub fn main() {
    let n = 4;
    assert_eq!(Solution::tribonacci(n), 4);

    let n = 25;
    assert_eq!(Solution::tribonacci(n), 1389537);
}
