// https://leetcode.com/problems/minimum-number-of-vertices-to-reach-all-nodes/

struct Solution;

impl Solution {
    pub fn find_smallest_set_of_vertices(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        use std::collections::HashSet;
        let mut vertices: HashSet<i32> = HashSet::from_iter((0..n).collect::<Vec<i32>>());
        for edge in edges.iter() {
            vertices.remove(&edge[1]);
        }

        return vertices.into_iter().collect::<Vec<i32>>();
    }
}

#[allow(dead_code)]
pub fn main() {
    let n = 6;
    let edges = vec![vec![0, 1], vec![0, 2], vec![2, 5], vec![3, 4], vec![4, 2]];
    assert_eq!(
        Solution::find_smallest_set_of_vertices(n, edges),
        vec![0, 3]
    )
}
