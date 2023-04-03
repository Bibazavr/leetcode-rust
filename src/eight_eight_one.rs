// https://leetcode.com/problems/boats-to-save-people/

struct Solution;

impl Solution {
    pub fn num_rescue_boats(mut people: Vec<i32>, limit: i32) -> i32 {
        people.sort_unstable();
        let mut max = people.len() - 1;
        let mut min = 0;
        let mut boat_count = 0;

        while min <= max {
            if people[min] + people[max] <= limit {
                min += 1;
            }
            boat_count += 1;
            if max == 0 {
                break;
            }
            max -= 1;
        }

        boat_count
    }
}

#[allow(dead_code)]
pub fn main() {
    let s1 = vec![3, 5, 3, 4];
    let s2 = 5;
    assert_eq!(Solution::num_rescue_boats(s1, s2), 4);

    let s1 = vec![1, 2];
    let s2 = 3;
    assert_eq!(Solution::num_rescue_boats(s1, s2), 1);
    let s1 = vec![1, 2];
    let s2 = 3;
    assert_eq!(Solution::num_rescue_boats(s1, s2), 1);
}
