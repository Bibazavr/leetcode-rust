// https://leetcode.com/problems/jump-game-iv/

struct Solution;

impl Solution {
    pub fn min_jumps(arr: Vec<i32>) -> i32 {
        let mut hash = std::collections::HashMap::new();
        for (i, &num) in arr.iter().enumerate() {
            hash.entry(num).or_insert(vec![]).push(i);
        }

        let mut visited = vec![false; arr.len()];
        visited[0] = true;

        let mut queue = std::collections::VecDeque::new();
        queue.push_back(0);

        let mut step = 0;
        let goal = arr.len() - 1;

        while !queue.is_empty() {
            for _ in 0..queue.len() {
                if let Some(i) = queue.pop_front() {
                    if i == goal {
                        return step;
                    }
                    if i > 0 && !visited[i - 1] {
                        visited[i - 1] = true;
                        queue.push_back(i - 1);
                    }
                    if i < goal && !visited[i + 1] {
                        visited[i + 1] = true;
                        queue.push_back(i + 1);
                    }

                    if let Some(neigh) = hash.get(&arr[i]) {
                        neigh.iter().for_each(|&idx| {
                            if !visited[idx] {
                                visited[idx] = true;
                                queue.push_back(idx)
                            }
                        });
                        hash.remove(&arr[i]);
                    }
                }
            }
            step += 1;
        }
        -1
    }
}

#[allow(dead_code)]
pub fn main() {
    let arr = vec![100, -23, -23, 404, 100, 23, 23, 23, 3, 404];
    assert_eq!(Solution::min_jumps(arr), 3)
}
