// https://leetcode.com/problems/insert-interval/

struct Solution;

impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        if intervals.len() == 0 {
            return vec![new_interval];
        }

        let mut answer = intervals.clone();
        let ps = intervals.partition_point(|interval| interval[1] < new_interval[0]);
        let pe = intervals.partition_point(|interval| interval[0] <= new_interval[1]);

        if ps == pe {
            answer.insert(pe, new_interval)
        } else {
            let new_interval = vec![
                intervals[ps][0].min(new_interval[0]),
                intervals[pe - 1][1].max(new_interval[1]),
            ];
            answer.drain(ps..pe);
            answer.insert(ps, new_interval);
        }
        answer
    }
}

#[allow(dead_code)]
pub fn main() {
    let intervals = vec![vec![1, 3], vec![6, 9]];
    let new_interval = vec![2, 5];
    assert_eq!(Solution::insert(intervals, new_interval), [[1, 5], [6, 9]]);

    let intervals = vec![];
    let new_interval = vec![5, 7];
    assert_eq!(Solution::insert(intervals, new_interval), [[5, 7]]);

    let intervals = vec![vec![1, 5]];
    let new_interval = vec![6, 8];
    assert_eq!(Solution::insert(intervals, new_interval), [[1, 5], [6, 8]]);
}
