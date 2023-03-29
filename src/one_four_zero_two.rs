// https://leetcode.com/problems/reducing-dishes/

struct Solution;

impl Solution {
    pub fn max_satisfaction(mut satisfaction: Vec<i32>) -> i32 {
        satisfaction.sort_unstable();
        let mut ans = 0;
        let mut acc = 0;
        for &x in satisfaction.iter().rev() {
            acc += x;
            if acc < 0 {
                break;
            }
            ans += acc;
        }
        ans
    }
}

#[allow(dead_code)]
pub fn main() {
    let satisfaction = vec![-1, -8, 0, 5, -9];
    assert_eq!(Solution::max_satisfaction(satisfaction), 14);
}
