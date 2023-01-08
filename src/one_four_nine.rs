// https://leetcode.com/problems/max-points-on-a-line/description/
use std::cmp::max;
use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn max_points(points: Vec<Vec<i32>>) -> i32 {
        if points.len() == 1 {
            return 1;
        }

        let mut max_points = 0;
        let mut slope_count = HashMap::new();

        for (i, point) in points.iter().enumerate() {
            slope_count.clear();

            let x1 = point[0];
            let y1 = point[1];

            let mut overlap = 0;
            let mut current_max = 0;

            for j in i + 1..points.len() {
                let x2 = points[j][0];
                let y2 = points[j][1];

                if x1 == x2 && y1 == y2 {
                    overlap += 1;
                    continue;
                }

                let slope = (if x1 == x2 {
                    f32::INFINITY
                } else {
                    (y2 - y1) as f32 / (x2 - x1) as f32
                } * (10000.0)) as i32;

                *slope_count.entry(slope).or_insert(0) += 1;

                current_max = max(current_max, *slope_count.get(&slope).unwrap());
            }

            max_points = max(max_points, current_max + overlap + 1)
        }

        max_points
    }
}

#[allow(dead_code)]
pub fn main() {
    let points = vec![vec![1, 1], vec![2, 2], vec![3, 3]];
    assert_eq!(Solution::max_points(points), 3);

    let points = vec![
        vec![0, 0],
        vec![4, 5],
        vec![7, 8],
        vec![8, 9],
        vec![5, 6],
        vec![3, 4],
        vec![1, 1],
    ];
    assert_eq!(Solution::max_points(points), 5);

    let points = vec![vec![2, 3], vec![3, 3], vec![-5, 3]];
    assert_eq!(Solution::max_points(points), 3);

    let points = vec![vec![0, 0], vec![1, -1], vec![1, 1]];
    assert_eq!(Solution::max_points(points), 2);

    let points = vec![
        vec![-424, -512],
        vec![-4, -47],
        vec![0, -23],
        vec![-7, -65],
        vec![7, 138],
        vec![0, 27],
        vec![-5, -90],
        vec![-106, -146],
        vec![-420, -158],
        vec![-7, -128],
        vec![0, 16],
        vec![-6, 9],
        vec![-34, 26],
        vec![-9, -166],
        vec![-570, -69],
        vec![-665, -85],
        vec![560, 248],
        vec![1, -17],
        vec![630, 277],
        vec![1, -7],
        vec![-287, -222],
        vec![30, 250],
        vec![5, 5],
        vec![-475, -53],
        vec![950, 187],
        vec![7, -6],
        vec![-700, -274],
        vec![3, 62],
        vec![-318, -390],
        vec![7, 19],
        vec![-285, -21],
        vec![-5, 4],
        vec![53, 37],
        vec![-5, -1],
        vec![-2, -33],
        vec![-95, 11],
        vec![4, 1],
        vec![8, 25],
        vec![700, 306],
        vec![1, 24],
        vec![-2, -6],
        vec![-35, -387],
        vec![-630, -245],
        vec![-328, -260],
        vec![-350, -129],
        vec![35, 299],
        vec![-380, -37],
        vec![-9, -9],
        vec![210, 103],
        vec![7, -5],
        vec![-3, -52],
        vec![-51, 23],
        vec![-8, -147],
        vec![-371, -451],
        vec![-1, -14],
        vec![-41, 6],
        vec![-246, -184],
        vec![350, 161],
        vec![-212, -268],
        vec![-140, -42],
        vec![-9, -4],
        vec![-7, 5],
        vec![10, 6],
        vec![-15, -191],
        vec![-7, -4],
        vec![318, 342],
        vec![-8, -71],
        vec![-68, 20],
        vec![6, 119],
        vec![6, 13],
        vec![-280, -100],
        vec![140, 74],
        vec![-760, -101],
        vec![0, -24],
        vec![-70, -13],
        vec![0, 2],
        vec![0, -9],
        vec![106, 98],
    ];
    assert_eq!(Solution::max_points(points), 14);
}
