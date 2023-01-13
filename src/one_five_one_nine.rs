// https://leetcode.com/problems/number-of-nodes-in-the-sub-tree-with-the-same-label/

struct Solution;

impl Solution {
    pub fn count_sub_trees(n: i32, edges: Vec<Vec<i32>>, labels: String) -> Vec<i32> {
        let mut adjacent = vec![Vec::new(); n as usize];
        for edge in edges.into_iter() {
            adjacent[edge[0] as usize].push(edge[1] as usize);
            adjacent[edge[1] as usize].push(edge[0] as usize);
        }

        let mut solution = vec![0; n as usize];
        let mut subtree = [0; 26];

        Self::dfs_sub_tree(
            &adjacent,
            labels.as_bytes(),
            &mut solution,
            &mut subtree,
            0,
            0,
        );
        solution
    }

    fn dfs_sub_tree(
        adjacent: &Vec<Vec<usize>>,
        labels: &[u8],
        solution: &mut Vec<i32>,
        subtree: &mut [i32; 26],
        node: usize,
        parent: usize,
    ) {
        solution[node] -= subtree[(labels[node] - b'a') as usize];

        subtree[(labels[node] - b'a') as usize] += 1;
        for &child in adjacent[node].iter() {
            if child != parent {
                Self::dfs_sub_tree(adjacent, labels, solution, subtree, child, node);
            }
        }
        solution[node] += subtree[(labels[node] - b'a') as usize];
    }
}

#[allow(dead_code)]
pub fn main() {
    let n = 7;

    let edges = vec![
        vec![0, 1],
        vec![0, 2],
        vec![1, 4],
        vec![1, 5],
        vec![2, 3],
        vec![2, 6],
    ];

    let labels = String::from("abaedcd");

    assert_eq!(
        Solution::count_sub_trees(n, edges, labels),
        vec![2, 1, 1, 1, 1, 1, 1]
    );

    let n = 4;

    let edges = vec![vec![0, 1], vec![1, 2], vec![0, 3]];

    let labels = String::from("bbbb");

    assert_eq!(
        Solution::count_sub_trees(n, edges, labels),
        vec![2, 1, 1, 1, 1, 1, 1]
    );
}
