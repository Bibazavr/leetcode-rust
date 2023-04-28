// https://leetcode.com/problems/similar-string-groups/
struct Solution;

use std::collections::HashSet;
use std::iter::FromIterator;

trait UnionFind {
    fn union(parents: &mut Vec<usize>, i: usize, j: usize);
    fn find(parents: &mut Vec<usize>, i: usize) -> usize;
}

impl UnionFind for Solution {
    fn union(parents: &mut Vec<usize>, i: usize, j: usize) {
        let pi = Solution::find(parents, i);
        let pj = Solution::find(parents, j);
        parents[pi] = pj;
    }
    fn find(parents: &mut Vec<usize>, i: usize) -> usize {
        if parents[i] != i {
            parents[i] = Solution::find(parents, parents[i]);
        }
        parents[i]
    }
}

impl Solution {
    pub fn is_similar_string(first: &String, second: &String) -> bool {
        match first
            .chars()
            .zip(second.chars())
            .filter(|(c1, c2)| c1 != c2)
            .collect::<Vec<_>>()
            .len()
        {
            0 | 2 => true,
            _ => false,
        }
    }
    pub fn num_similar_groups(strs: Vec<String>) -> i32 {
        let mut parents = (0..strs.len()).collect::<Vec<_>>();
        let n = strs.len();

        for i in 0..n {
            for j in i + 1..n {
                if Solution::is_similar_string(&strs[i], &strs[j]) {
                    Solution::union(&mut parents, i, j);
                }
            }
        }
        for i in 0..n {
            Solution::find(&mut parents, i);
        }
        HashSet::<usize>::from_iter(parents).len() as i32
    }
}

#[allow(dead_code)]
pub fn main() {
    let strs = vec![
        String::from("tars"),
        String::from("rats"),
        String::from("arts"),
        String::from("star"),
    ];

    assert_eq!(Solution::num_similar_groups(strs), 2);
}
