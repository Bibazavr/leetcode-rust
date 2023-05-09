// https://leetcode.com/problems/spiral-matrix/

struct Solution;

impl Solution {
    pub fn spiral_order(mut matrix: Vec<Vec<i32>>) -> Vec<i32> {
        // direction: up, right, down, left
        let mut direction = [(-1_i32, 0_i32), (0, 1), (1, 0), (0, -1)].iter().cycle();
        let row = matrix.len();
        let col = matrix[0].len();
        let mut result = Vec::with_capacity(row * col);

        let mut i: i32 = 0;
        let mut j: i32 = -1; // prefix: -1 + 1 = 0 -> (i,j) = (0,0)
        while result.len() != row * col {
            let (ix, jx) = direction.next().unwrap();
            while {
                i += ix;
                j += jx;

                if i >= 0
                    && i < row as i32
                    && j >= 0
                    && j < col as i32
                    && matrix[i as usize][j as usize] != 404
                {
                    true
                } else {
                    i -= ix;
                    j -= jx;
                    false
                }
            } {
                result.push(matrix[i as usize][j as usize]);
                matrix[i as usize][j as usize] = 404;
            }
        }
        result
    }
}
#[allow(dead_code)]
pub fn main() {
    let matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];

    assert_eq!(
        Solution::spiral_order(matrix),
        vec![1, 2, 3, 6, 9, 8, 7, 4, 5]
    );
}
