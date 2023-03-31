// https://leetcode.com/problems/number-of-ways-of-cutting-a-pizza/
// BIBA_STRUCTURE
struct Solution;

impl Solution {
    pub fn ways(pizza: Vec<String>, k: i32) -> i32 {
        num_ways_of_cutting_pizza(pizza, k)
    }
}

const MOD: usize = 1000000007;

pub enum Cell {
    Apple,
    Other,
}

impl From<char> for Cell {
    fn from(value: char) -> Self {
        match value {
            'A' => Cell::Apple,
            '.' => Cell::Other,
            _ => unimplemented!(),
        }
    }
}

impl From<u8> for Cell {
    fn from(value: u8) -> Self {
        match value {
            b'A' => Cell::Apple,
            b'.' => Cell::Other,
            _ => unimplemented!(),
        }
    }
}

impl From<&Cell> for usize {
    fn from(value: &Cell) -> Self {
        match value {
            Cell::Apple => 1,
            Cell::Other => 0,
        }
    }
}

pub fn num_ways_of_cutting_pizza(pizza: Vec<String>, k: i32) -> i32 {
    let pizza = pizza
        .into_iter()
        .map(|row| row.chars().map(|cell| cell.into()).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    num_ways(&pizza, k as usize) as i32
}

pub fn num_ways(pizza: &Vec<Vec<Cell>>, k: usize) -> usize {
    let rows = pizza.len();
    let cols = pizza[0].len();

    let mut dp = vec![vec![vec![0; cols + 1]; rows + 1]; k];
    let counts = Count::new(pizza);

    for row in (0..rows).rev() {
        for col in (0..cols).rev() {
            dp[0][row][col] = if counts.get_count((row, col)) > 0 {
                1
            } else {
                0
            };
        }
    }

    for remain in 1..k {
        for row in 0..rows {
            for col in 0..cols {
                for next_row in (row + 1)..rows {
                    if counts.get_count((row, col)) > counts.get_count((next_row, col)) {
                        dp[remain][row][col] += dp[remain - 1][next_row][col];
                        dp[remain][row][col] %= MOD;
                    }
                }

                for next_col in (col + 1)..cols {
                    if counts.get_count((row, col)) > counts.get_count((row, next_col)) {
                        dp[remain][row][col] += dp[remain - 1][row][next_col];
                        dp[remain][row][col] %= MOD;
                    }
                }
            }
        }
    }

    dp[k - 1][0][0]
}

struct Count {
    rows: usize,
    cols: usize,
    counts: Vec<Vec<usize>>,
}

impl Count {
    fn new(matrix: &Vec<Vec<Cell>>) -> Self {
        assert!(!matrix.is_empty());

        let rows = matrix.len();
        let cols = matrix[0].len();

        assert!(cols > 0);

        let mut counts = vec![vec![0; cols + 1]; rows + 1];

        for i in (0..rows).rev() {
            for j in (0..cols).rev() {
                let count: usize = (&matrix[i][j]).into();

                counts[i][j] = count + counts[i + 1][j] + counts[i][j + 1] - counts[i + 1][j + 1];
            }
        }

        Self { rows, cols, counts }
    }

    /// Get count of apples in submatrix starting at top-left corner till bottom-right end
    fn get_count(&self, (x, y): (usize, usize)) -> usize {
        self.get_count_in_range((x, y), (self.rows - 1, self.cols - 1))
    }

    fn get_count_in_range(&self, (x1, y1): (usize, usize), (x2, y2): (usize, usize)) -> usize {
        self.check_validity((x1, y1), (x2, y2));

        let x2 = x2 + 1;
        let y2 = y2 + 1;

        self.counts[x2][y2] + self.counts[x1][y1] - self.counts[x2][y1] - self.counts[x1][y2]
    }

    fn check_validity(&self, (x1, y1): (usize, usize), (x2, y2): (usize, usize)) {
        assert!(x1 < self.rows);
        assert!(x2 < self.rows);
        assert!(y1 < self.cols);
        assert!(y2 < self.cols);
        assert!(x2 >= x1 && y2 >= y1);
    }
}

#[allow(dead_code)]
pub fn main() {
    let pizza = vec![
        String::from("A.."),
        String::from("AAA"),
        String::from("..."),
    ];
    let k = 3;

    assert_eq!(Solution::ways(pizza, k), 3);
}
