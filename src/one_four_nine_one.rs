// https://leetcode.com/problems/average-salary-excluding-the-minimum-and-maximum-salary/

struct Solution;

impl Solution {
    pub fn average(salary: Vec<i32>) -> f64 {
        let mut min = i32::MAX;
        let mut max = 0;
        let mut sum = 0;
        let n = salary.len();

        for v in salary {
            max = max.max(v);
            min = min.min(v);
            sum += v;
        }

        return ((sum - min - max) as f64) / (n as f64 - 2.0);
    }
}

#[allow(dead_code)]
pub fn main() {
    let s = vec![4000, 3000, 1000, 2000];
    assert_eq!(Solution::average(s), 2500.00000);
}
