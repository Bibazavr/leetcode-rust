// https://leetcode.com/problems/can-place-flowers/

struct Solution;

impl Solution {
    pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
        flowerbed
            .iter()
            .chain(std::iter::once(&0))
            .fold((1, 0), |(consecutive_zeros, open_pots), &pot| {
                if pot == 1 {
                    (0, open_pots)
                } else if consecutive_zeros == 2 {
                    (1, open_pots + 1)
                } else {
                    (consecutive_zeros + 1, open_pots)
                }
            })
            .1
            >= n
    }
}

#[allow(dead_code)]
pub fn main() {
    let flowerbed = vec![1, 0, 0, 0, 0];
    let n = 2;
    assert_eq!(Solution::can_place_flowers(flowerbed, n), true);

    let flowerbed = vec![1, 0, 0, 0, 1];
    let n = 2;
    assert_eq!(Solution::can_place_flowers(flowerbed, n), false);

    let flowerbed = vec![1, 0, 0, 0, 1];
    let n = 1;
    assert_eq!(Solution::can_place_flowers(flowerbed, n), true);
}
