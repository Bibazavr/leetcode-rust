// https://leetcode.com/problems/maximum-value-of-k-coins-from-piles/

struct Solution;

impl Solution {
    pub fn max_value_of_coins(piles: Vec<Vec<i32>>, k: i32) -> i32 {
        let n = piles.len();
        let k = k as usize;
        let mut prev = vec![0; k + 1];
        let mut curr = vec![0; k + 1];

        for i in 1..n + 1 {
            for coins in 1..k + 1 {
                let not_pick = prev[coins];
                let mut pick = 0;

                let mut coin_sum = 0;
                for j in 0..piles[i - 1].len() {
                    coin_sum += piles[i - 1][j];

                    if coins >= j + 1 {
                        pick = std::cmp::max(pick, coin_sum + prev[coins - j - 1]);
                    }
                }

                curr[coins] = std::cmp::max(pick, not_pick);
            }
            prev.clone_from_slice(&curr);
        }

        prev[k]
    }
}

#[allow(dead_code)]
pub fn main() {
    let piles = vec![vec![1, 100, 3], vec![7, 8, 9]];
    let k = 2;

    assert_eq!(Solution::max_value_of_coins(piles, k), 101);
}
