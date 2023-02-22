// https://leetcode.com/problems/capacity-to-ship-packages-within-d-days/

struct Solution;

impl Solution {
    pub fn ship_within_days(weights: Vec<i32>, days: i32) -> i32 {
        let mut max_capacity = 0;
        let mut min_capacity = 0;

        for weight in weights.iter() {
            max_capacity += weight;
            min_capacity = std::cmp::max(min_capacity, *weight);
        }

        while min_capacity < max_capacity {
            let capacity = (min_capacity + max_capacity) >> 1;
            let mut current_load = 0;
            let mut current_days = 1;
            for &weight in weights.iter() {
                current_load += weight;
                if current_load > capacity {
                    current_days += 1;
                    current_load = weight;
                }
            }

            if current_days > days {
                min_capacity = capacity + 1;
            } else {
                max_capacity = capacity;
            }
        }

        min_capacity
    }
}

#[allow(dead_code)]
pub fn main() {
    let weights = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let days = 5;
    assert_eq!(Solution::ship_within_days(weights, days), 15);
}
