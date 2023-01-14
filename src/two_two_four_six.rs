// https://leetcode.com/problems/longest-path-with-different-adjacent-characters/

struct Solution;

impl Solution {
    pub fn longest_path(parent: Vec<i32>, s: String) -> i32 {
        let mut ans = 0;
        let n = parent.len() as usize;
        let s = s.into_bytes();

        let mut subtree = vec![Vec::new(); n];

        for i in 1..n {
            subtree[parent[i] as usize].push(i);
        }

        Self::dfs(0, 0, &subtree, &s, &mut ans);

        ans
    }

    fn dfs(
        node: usize,
        parent: usize,
        subtree: &Vec<Vec<usize>>,
        s: &Vec<u8>,
        ans: &mut i32,
    ) -> i32 {
        let mut max = 0;
        for child in subtree[node].iter() {
            if child == &parent {
                continue;
            }
            let part = Self::dfs(*child, node, subtree, s, ans);
            *ans = i32::max(*ans, part + max + 1);

            max = max.max(part);
        }

        max += 1;

        if max > *ans {
            *ans = max;
        }

        if s[parent] == s[node] {
            return 0;
        }

        max
    }
}

#[allow(dead_code)]
pub fn main() {
    let lists = vec![-1, 0, 0, 1, 1, 2];
    let s = String::from("abacbe");

    assert_eq!(Solution::longest_path(lists, s), 3);

    let lists = vec![-1, 0, 0, 0];
    let s = String::from("aabc");

    assert_eq!(Solution::longest_path(lists, s), 3);
}
