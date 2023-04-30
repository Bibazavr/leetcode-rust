// https://leetcode.com/problems/remove-max-number-of-edges-to-keep-graph-fully-traversable/

struct Solution;

impl Solution {
    pub fn max_num_edges_to_remove(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let mut res = 0;
        let n = n as usize;
        let (mut ufa, mut ufb) = (UnionFind::new(n), UnionFind::new(n));

        for edge in edges.iter() {
            let (t, u, v) = (edge[0], edge[1] as usize - 1, edge[2] as usize - 1);

            if t == 3 {
                if !ufa.union(u, v) || !ufb.union(u, v) {
                    res += 1;
                }
            }
        }

        for edge in edges.iter() {
            let (t, u, v) = (edge[0], edge[1] as usize - 1, edge[2] as usize - 1);

            if t == 1 && !ufa.union(u, v) {
                res += 1;
            }
            if t == 2 && !ufb.union(u, v) {
                res += 1;
            }
        }

        if ufa.count == 1 && ufb.count == 1 {
            res
        } else {
            -1
        }
    }
}

pub struct UnionFind {
    root: Vec<usize>,
    count: usize,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        let mut root = vec![0; n];
        for i in 0..n {
            root[i] = i;
        }

        let count = n;

        UnionFind { root, count }
    }

    pub fn find(&mut self, x: usize) -> usize {
        if self.root[x] != x {
            self.root[x] = Self::find(self, self.root[x]);
        }

        self.root[x]
    }

    pub fn union(&mut self, x: usize, y: usize) -> bool {
        let root_x = Self::find(self, x);
        let root_y = Self::find(self, y);

        if root_x == root_y {
            return false;
        }

        if root_x < root_y {
            self.root[root_y] = root_x;
        } else {
            self.root[root_x] = root_y;
        }

        self.count -= 1;

        true
    }
}
#[allow(dead_code)]
pub fn main() {
    let n = 4;

    let edges = vec![
        vec![3, 1, 2],
        vec![3, 2, 3],
        vec![1, 1, 3],
        vec![1, 2, 4],
        vec![1, 1, 2],
        vec![2, 3, 4],
    ];

    assert_eq!(Solution::max_num_edges_to_remove(n, edges), 2);
}
