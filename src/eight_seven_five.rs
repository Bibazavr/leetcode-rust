// https://leetcode.com/problems/koko-eating-bananas/
struct Solution;

impl Solution {
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        let mut max_speed = piles.iter().max().unwrap_or(&i32::MAX).clone();
        let mut min_speed = 1;

        while min_speed < max_speed {
            let speed = (min_speed + max_speed) >> 1;
            let hours = piles
                .iter()
                .fold(0, |subtotal, &pile| subtotal + ((pile + speed - 1) / speed));
            if hours > h {
                min_speed = speed + 1;
            } else {
                max_speed = speed;
            }
        }

        min_speed
    }
}
#[allow(dead_code)]
pub fn main() {
    let piles = vec![3, 6, 7, 11];
    let h = 8;

    assert_eq!(Solution::min_eating_speed(piles, h), 4);
}
