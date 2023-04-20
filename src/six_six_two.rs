// https://leetcode.com/problems/maximum-width-of-binary-tree/

struct Solution;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[allow(dead_code)]
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn width_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut v = Vec::new();
        Solution::dfs(&root, 0, 0, &mut v) as i32
    }
    fn dfs(
        node: &Option<Rc<RefCell<TreeNode>>>,
        depth: usize,
        index: usize,
        v: &mut Vec<usize>,
    ) -> usize {
        if let Some(n) = node {
            if depth >= v.len() {
                v.push(index);
            }
            std::cmp::max(
                index - v[depth] + 1,
                std::cmp::max(
                    Solution::dfs(&n.borrow().left, depth + 1, index * 2, v),
                    Solution::dfs(&n.borrow().right, depth + 1, index * 2 + 1, v),
                ),
            )
        } else {
            0
        }
    }
}

#[allow(dead_code)]
pub fn main() {
    let root = Option::from(Rc::from(RefCell::from(TreeNode {
        val: 1,
        left: Option::from(Rc::from(RefCell::from(TreeNode {
            val: 3,
            left: Option::from(Rc::from(RefCell::from(TreeNode {
                val: 5,
                left: None,
                right: None,
            }))),
            right: Option::from(Rc::from(RefCell::from(TreeNode {
                val: 3,
                left: None,
                right: None,
            }))),
        }))),
        right: Option::from(Rc::from(RefCell::from(TreeNode {
            val: 2,
            left: None,
            right: Option::from(Rc::from(RefCell::from(TreeNode {
                val: 4,
                left: None,
                right: None,
            }))),
        }))),
    })));
    assert_eq!(Solution::width_of_binary_tree(root), 4);
}
