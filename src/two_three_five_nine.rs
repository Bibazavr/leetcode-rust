// https://leetcode.com/problems/find-closest-node-to-given-two-nodes/

struct Solution;

impl Solution {
    pub fn closest_meeting_node(edges: Vec<i32>, node1: i32, node2: i32) -> i32 {
        let mut closest_meeting_node_index = -1;
        let mut min_distance = i32::MAX;
        let n = edges.len();

        let mut dist1 = vec![-1; n];
        let mut dist2 = vec![-1; n];

        Self::dfs(&edges, 0, node1 as usize, &mut dist1);
        Self::dfs(&edges, 0, node2 as usize, &mut dist2);

        println!("{:?} {:?}", dist1, dist2);

        for i in 0..n {
            let one = dist1[i];
            let two = dist2[i];

            if one == -1 || two == -1 {
                continue;
            }

            let current_distance = std::cmp::max(dist1[i], dist2[i]);
            if current_distance < min_distance {
                closest_meeting_node_index = i as i32;
                min_distance = current_distance
            }
        }
        closest_meeting_node_index
    }

    fn dfs(edges: &Vec<i32>, depth: i32, target: usize, dist: &mut Vec<i32>) {
        if dist[target] != -1 {
            return;
        }
        dist[target] = depth;
        if edges[target] == -1 {
            return;
        }
        let node = edges[target] as usize;
        Self::dfs(edges, depth + 1, node, dist);
    }
}

#[allow(dead_code)]
pub fn main() {
    let edges = vec![2, 2, 3, -1];
    let node1 = 0;
    let node2 = 1;

    assert_eq!(Solution::closest_meeting_node(edges, node1, node2), 2);

    let edges = vec![4, 4, 4, 5, 1, 2, 2];
    let node1 = 1;
    let node2 = 1;

    assert_eq!(Solution::closest_meeting_node(edges, node1, node2), 1);
}
