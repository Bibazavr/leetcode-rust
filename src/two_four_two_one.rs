// https://leetcode.com/problems/number-of-good-paths/

struct Solution;

impl Solution {
    pub fn number_of_good_paths(vals: Vec<i32>, edges: Vec<Vec<i32>>) -> i32 {
        use std::collections::{BTreeMap, HashMap};
        let n = vals.len();

        let mut adjacent = vec![Vec::new(); vals.len() as usize];
        for edge in edges.into_iter() {
            if vals[edge[0] as usize] >= vals[edge[1] as usize] {
                adjacent[edge[0] as usize].push(edge[1] as usize);
            } else {
                adjacent[edge[1] as usize].push(edge[0] as usize);
            }
        }

        let mut val_to_idx: BTreeMap<i32, Vec<usize>> = BTreeMap::new();
        for (i, val) in vals.into_iter().enumerate() {
            val_to_idx.entry(val).or_insert(vec![]).push(i);
        }

        let mut uf: Vec<_> = (0..n).into_iter().collect();
        fn uf_find(uf: &mut Vec<usize>, a: usize) -> usize {
            let mut b = a;
            while uf[b] != b {
                b = uf[b];
            }
            uf[a] = b;
            b
        }
        fn uf_union(uf: &mut Vec<usize>, mut a: usize, mut b: usize) {
            a = uf_find(uf, a);
            b = uf_find(uf, b);
            uf[a] = b;
        }

        let mut answer = n;
        for (_val, idx_arr) in val_to_idx {
            for &idx in &idx_arr {
                for &node in &adjacent[idx] {
                    uf_union(&mut uf, idx, node);
                }
            }

            let mut group: HashMap<usize, usize> = HashMap::new();
            for idx in idx_arr {
                group
                    .entry(uf_find(&mut uf, idx))
                    .and_modify(|count| *count += 1)
                    .or_insert(1);
            }
            for (_, count) in group {
                answer += count * (count - 1) / 2;
            }
        }
        answer as _
    }
}

#[allow(dead_code)]
pub fn main() {
    let vals = vec![1, 3, 2, 1, 3];
    let edges = vec![vec![0, 1], vec![0, 2], vec![2, 3], vec![2, 4]];
    assert_eq!(Solution::number_of_good_paths(vals, edges), 6);

    let vals = vec![2, 1, 1];
    let edges = vec![vec![0, 1], vec![2, 0]];
    assert_eq!(Solution::number_of_good_paths(vals, edges), 3);
}
