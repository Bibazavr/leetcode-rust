// https://leetcode.com/problems/find-the-longest-valid-obstacle-course-at-each-position/

struct Solution;

impl Solution {
    pub fn longest_obstacle_course_at_each_position(obstacles: Vec<i32>) -> Vec<i32> {
        let mut template = vec![];
        let mut result = vec![];

        for n in obstacles {
            match template.last() {
                Some(&last) if last > n => (),
                _ => {
                    template.push(n);
                    result.push(template.len() as i32);
                    continue;
                }
            }

            let mut l = 0;
            let mut r = template.len();
            while l < r {
                let mid = l + (r - l) / 2;
                if template[mid] <= n {
                    l = mid + 1;
                } else {
                    r = mid;
                }
            }
            template[l] = n;
            result.push(l as i32 + 1);
        }
        result
    }
}

#[allow(dead_code)]
pub fn main() {
    let obstacles = vec![1, 2, 3, 2];
    assert_eq!(
        Solution::longest_obstacle_course_at_each_position(obstacles),
        vec![1, 2, 3, 3]
    );
}
