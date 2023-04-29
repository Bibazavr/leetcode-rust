// https://leetcode.com/problems/checking-existence-of-edge-length-limited-paths/

struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> UnionFind {
        return UnionFind {
            parent: (0..n).collect(),
            size: vec![1; n],
        };
    }

    fn find(&mut self, p: usize) -> usize {
        if p == self.parent[p] {
            return p;
        }

        self.parent[p] = self.find(self.parent[p]);
        return self.parent[p];
    }

    fn union(&mut self, mut p: usize, mut q: usize) {
        p = self.find(p);
        q = self.find(q);

        if p != q {
            let (size_p, size_q) = (self.size[p], self.size[q]);
            if size_p > size_q {
                self.size[p] += size_q;
                self.parent[q] = p;
            } else {
                self.size[q] += size_p;
                self.parent[p] = q;
            }
        }
    }
}

struct Solution;

impl Solution {
    pub fn distance_limited_paths_exist(
        n: i32,
        mut edges: Vec<Vec<i32>>,
        queries: Vec<Vec<i32>>,
    ) -> Vec<bool> {
        let mut result = vec![false; queries.len()];
        let mut uf = UnionFind::new(n as usize);
        let mut last_pos = 0;

        let mut q_idx: Vec<_> = (0..queries.len()).collect();
        q_idx.sort_unstable_by_key(|&i| queries[i][2]);
        edges.sort_unstable_by_key(|v| v[2]);

        for i in q_idx {
            while last_pos < edges.len() && edges[last_pos][2] < queries[i][2] {
                uf.union(edges[last_pos][0] as usize, edges[last_pos][1] as usize);
                last_pos += 1;
            }

            if uf.find(queries[i][0] as usize) == uf.find(queries[i][1] as usize) {
                result[i] = true;
            }
        }

        return result;
    }
}

#[allow(dead_code)]
pub fn main() {
    let n = 3;
    let edge_list = vec![vec![0, 1, 2], vec![1, 2, 4], vec![2, 0, 8], vec![1, 0, 16]];
    let queries = vec![vec![0, 1, 2], vec![0, 2, 5]];
    assert_eq!(
        Solution::distance_limited_paths_exist(n, edge_list, queries),
        vec![false, true]
    );
}
