// https://leetcode.com/problems/count-unreachable-pairs-of-nodes-in-an-undirected-graph/

struct Solution;

use std::collections::HashMap;

struct UnionFind {
    parents: Vec<usize>,
    sizes: HashMap<usize, usize>,
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        Self {
            parents: (0..n).collect(),
            sizes: HashMap::new(),
        }
    }

    pub fn find(&mut self, mut k: usize) -> usize {
        let mut path = Vec::new();
        loop {
            let q = self.parents[k];
            if k == q {
                break;
            }
            path.push(k);
            k = q;
        }
        for q in path {
            self.parents[q] = k;
        }
        k
    }

    pub fn union(&mut self, k: usize, q: usize) -> bool {
        let mut x = self.find(k);
        let mut y = self.find(q);
        if x == y {
            return false;
        }
        let x_size = self.sizes.remove(&x).unwrap_or(1);
        let y_size = self.sizes.remove(&y).unwrap_or(1);
        if x_size < y_size {
            std::mem::swap(&mut x, &mut y);
        }
        self.parents[y] = x;
        self.sizes.insert(x, x_size + y_size);
        true
    }

    pub fn unconnected_pairs(&self) -> Option<usize> {
        let mut n = self.parents.len();
        let mut res = 0usize;
        for &size in self.sizes.values() {
            n -= size;
            res = res.checked_add(n.checked_mul(size)?)?;
        }
        if n > 0 {
            res = res.checked_add(n.checked_mul(n - 1)? / 2)?;
        }
        Some(res)
    }
}

impl Solution {
    pub fn count_pairs(n: i32, edges: Vec<Vec<i32>>) -> i64 {
        let n = n as usize;
        let mut uf = UnionFind::new(n);
        for edge in edges {
            uf.union(edge[0] as usize, edge[1] as usize);
        }
        uf.unconnected_pairs().unwrap() as i64
    }
}

#[allow(dead_code)]
pub fn main() {
    let n = 3;
    let edges = vec![vec![0, 1], vec![0, 2], vec![1, 2]];
    assert_eq!(Solution::count_pairs(n, edges), 0);
}
