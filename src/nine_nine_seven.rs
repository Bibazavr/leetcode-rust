// https://leetcode.com/problems/find-the-town-judge/description/

struct Solution;

impl Solution {
    pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
        if n == 1 {
            return n;
        }
        let mut counter = vec![0; (n + 1) as usize];
        trust.iter().for_each(|pair| {
            counter[pair[0] as usize] -= 1;
            counter[pair[1] as usize] += 1;
        });

        counter
            .iter()
            .position(|&num| num == n - 1)
            .map_or(-1, |i| i as i32)
    }
}

#[allow(dead_code)]
pub fn main() {
    let n = 2;
    let trust = vec![vec![1, 2]];
    assert_eq!(Solution::find_judge(n, trust), 2);
}
