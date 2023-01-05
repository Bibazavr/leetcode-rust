struct Solution;

impl Solution {
    pub fn find_min_arrow_shots(mut points: Vec<Vec<i32>>) -> i32 {
        points.sort_unstable_by(|l, r| l[1].cmp(&r[1]));
        let mut end = points[0][1];
        let mut sum = 1;
        for p in &points[1..] {
            let (x, y) = (p[0], p[1]);
            if end < x {
                sum += 1;
                end = y;
            }
        }
        sum
    }
}

#[allow(dead_code)]
pub fn main() {
    let points = vec![vec![10, 16], vec![2, 8], vec![1, 6], vec![7, 12]];
    let answer = Solution::find_min_arrow_shots(points);
    assert_eq!(answer, 2);
    println!("{:?}", answer)
}
