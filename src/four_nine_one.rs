// https://leetcode.com/problems/non-decreasing-subsequences/

struct Solution;

impl Solution {
    pub fn find_subsequences(nums: Vec<i32>) -> Vec<Vec<i32>> {
        use std::collections::HashSet;

        let mut res = HashSet::new();
        let mut subsequence: Vec<i32> = vec![];

        fn back_track(
            res: &mut HashSet<Vec<i32>>,
            nums: &Vec<i32>,
            i: usize,
            subsequence: &mut Vec<i32>,
        ) {
            if subsequence.len() > 1 {
                res.insert(subsequence.to_vec());
            }

            if i == nums.len() {
                return;
            }

            back_track(res, nums, i + 1, subsequence);

            match subsequence.last() {
                Some(&a) if a > nums[i] => (),
                _ => {
                    subsequence.push(nums[i]);
                    back_track(res, nums, i + 1, subsequence);
                    subsequence.pop();
                }
            }
        }

        back_track(&mut res, &nums, 0, &mut subsequence);

        return res.into_iter().collect();
    }
}

#[allow(dead_code)]
pub fn main() {
    let s = vec![4, 4, 3, 2, 1];
    assert_eq!(Solution::find_subsequences(s), vec![vec![4, 4],]);

    let s = vec![4, 6, 7, 7];
    assert_eq!(
        Solution::find_subsequences(s),
        vec![
            vec![4, 6],
            vec![4, 6, 7],
            vec![4, 6, 7, 7],
            vec![4, 7],
            vec![4, 7, 7],
            vec![6, 7],
            vec![6, 7, 7],
            vec![7, 7]
        ]
    );
}
