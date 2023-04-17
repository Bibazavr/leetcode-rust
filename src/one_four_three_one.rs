// https://leetcode.com/problems/kids-with-the-greatest-number-of-candies/

struct Solution;

impl Solution {
    pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
        let best = candies.iter().max().unwrap_or(&0) - extra_candies;
        candies.into_iter().map(|x| x >= best).collect()
    }
}

#[allow(dead_code)]
pub fn main() {
    let candies = vec![2, 3, 5, 1, 3];
    let extra_candies = 3;
    assert_eq!(
        Solution::kids_with_candies(candies, extra_candies),
        vec![true, true, true, false, true]
    );
}
