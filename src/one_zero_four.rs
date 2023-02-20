// https://leetcode.com/problems/maximum-depth-of-binary-tree/

struct Solution;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    #[allow(dead_code)]
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
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::dfs(root, 0)
    }
    pub fn dfs(root: Option<Rc<RefCell<TreeNode>>>, mut count: i32) -> i32 {
        if let Some(node_pointer) = root {
            count += 1;
            let node = node_pointer.borrow();
            let left = count.max(Self::dfs(node.left.clone(), count));
            let right = count.max(Self::dfs(node.right.clone(), count));
            count = left.max(right)
        }
        count
    }
}

#[allow(dead_code)]
pub fn main() {
    let root = Option::from(Rc::from(RefCell::from(TreeNode {
        val: 3,
        left: Option::from(Rc::from(RefCell::from(TreeNode {
            val: 9,
            left: None,
            right: None,
        }))),

        right: Option::from(Rc::from(RefCell::from(TreeNode {
            val: 20,
            left: Option::from(Rc::from(RefCell::from(TreeNode {
                val: 15,
                left: None,
                right: None,
            }))),
            right: Option::from(Rc::from(RefCell::from(TreeNode {
                val: 7,
                left: None,
                right: None,
            }))),
        }))),
    })));
    assert_eq!(Solution::max_depth(root), 3)
}
