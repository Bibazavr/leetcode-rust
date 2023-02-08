// https://leetcode.com/problems/jump-game-ii/
struct Solution;

impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        nums.iter()
            .enumerate()
            .skip(1)
            .fold(
                (if nums.len() == 1 { 0 } else { 1 }, nums[0], nums[0]),
                |(hops, max_reach, prev_max_reach), (idx, val)| {
                    if idx <= prev_max_reach as usize {
                        (hops, max_reach.max(idx as i32 + val), prev_max_reach)
                    } else {
                        (hops + 1, max_reach.max(idx as i32 + val), max_reach)
                    }
                },
            )
            .0
    }
}

#[allow(dead_code)]
pub fn main() {
    let nums = vec![2, 3, 1, 1, 4];

    assert_eq!(Solution::jump(nums), 2);
}
