// https://leetcode.com/problems/minimum-time-to-complete-trips/
struct Solution;

impl Solution {
    pub fn minimum_time(time: Vec<i32>, total_trips: i32) -> i64 {
        let total_trips = total_trips as i64;
        let mut max_time = time.iter().min().unwrap_or(&i32::MIN).clone() as i64 * total_trips;
        let mut min_time = 1 as i64;

        while min_time < max_time {
            let mid_time = (min_time + max_time) / 2;
            let trips = time.iter().fold(0, |subtotal, &trip_time| {
                subtotal + mid_time / trip_time as i64
            });

            if trips < total_trips {
                min_time = mid_time + 1;
            } else {
                max_time = mid_time;
            }
        }

        min_time
    }
}

#[allow(dead_code)]
pub fn main() {
    let time = vec![1, 2, 3];
    let total_trips = 5;
    assert_eq!(Solution::minimum_time(time, total_trips), 3);
}
