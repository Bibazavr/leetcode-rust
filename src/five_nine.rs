// https://leetcode.com/problems/spiral-matrix-ii/

struct Solution;

impl Solution {
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        let mut ans = vec![vec![0; n as usize]; n as usize];

        let (mut left, mut right, mut top, mut down) = (0, n - 1, 0, n - 1);
        let mut count = 1;

        while left <= right && top <= down {
            // top row.
            for i in left..=right {
                ans[top as usize][i as usize] = count;
                count += 1;
            }
            top += 1;

            // right column.
            for i in top..=down {
                ans[i as usize][right as usize] = count;
                count += 1;
            }
            right -= 1;

            // down row.
            for i in (left..=right).rev() {
                ans[down as usize][i as usize] = count;
                count += 1;
            }
            down -= 1;

            // left column.
            for i in (top..=down).rev() {
                ans[i as usize][left as usize] = count;
                count += 1;
            }
            left += 1;
        }

        return ans;
    }
}

#[allow(dead_code)]
pub fn main() {
    let n = 3;
    assert_eq!(
        Solution::generate_matrix(n),
        vec![vec![1, 2, 3], vec![8, 9, 4], vec![7, 6, 5]]
    );
}
