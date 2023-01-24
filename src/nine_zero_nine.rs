// https://leetcode.com/problems/snakes-and-ladders/

struct Solution;

impl Solution {
    pub fn snakes_and_ladders(board: Vec<Vec<i32>>) -> i32 {
        let n = board.len();
        let n_square = n * n;
        let mut board_flatten = vec![0];

        for i in 1..=n * n {
            let (x, y) = Self::convert_to_xy(n, i);
            board_flatten.push(board[x][y])
        }

        let mut seen = vec![];
        let mut queue = std::collections::VecDeque::from([(1, 0)]);

        while let Some((mut cur, step)) = queue.pop_front() {
            if board_flatten[cur] != -1 {
                cur = board_flatten[cur] as usize;
            }

            if cur == n_square {
                return step;
            }

            for next in (cur + 1)..=(cur + 6).min(n * n) {
                if !seen.contains(&next) {
                    seen.push(next);
                    queue.push_back((next, step + 1));
                }
            }
        }

        return -1;
    }

    fn convert_to_xy(n: usize, mut position: usize) -> (usize, usize) {
        position -= 1;
        if position / n % 2 == 0 {
            (n - position / n - 1, position % n)
        } else {
            (n - position / n - 1, n - position % n - 1)
        }
    }
}

#[allow(dead_code)]
pub fn main() {
    let board = vec![
        vec![-1, -1, -1, -1, -1, -1],
        vec![-1, -1, -1, -1, -1, -1],
        vec![-1, -1, -1, -1, -1, -1],
        vec![-1, 35, -1, -1, 13, -1],
        vec![-1, -1, -1, -1, -1, -1],
        vec![-1, 15, -1, -1, -1, -1],
    ];
    assert_eq!(Solution::snakes_and_ladders(board), 4);

    let board = vec![
        vec![-1, 1, 2, -1],
        vec![2, 13, 15, -1],
        vec![-1, 10, -1, -1],
        vec![-1, 6, 2, 8],
    ];
    assert_eq!(Solution::snakes_and_ladders(board), 2);
}
