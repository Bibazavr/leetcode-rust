// https://leetcode.com/problems/same-tree/

struct Solution;

// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}
//
impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

use std::borrow::Borrow;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match (p, q) {
            (None, None) => true,
            (None, Some(_)) => false,
            (Some(_), None) => false,
            (Some(m), Some(n)) => {
                let m = RefCell::borrow(&m);
                let n = RefCell::borrow(&n);
                m.val == n.val
                    && Self::is_same_tree(n.left.clone(), m.left.clone())
                    && Self::is_same_tree(n.right.clone(), m.right.clone())
            }
        }
    }
}

#[allow(dead_code)]
pub fn main() {
    let tree1 = Option::from(Rc::from(RefCell::from(TreeNode {
        val: 1,
        left: None,
        right: None,
    })));

    let tree2 = Option::from(Rc::from(RefCell::from(TreeNode {
        val: 1,
        left: None,
        right: None,
    })));

    assert_eq!(Solution::is_same_tree(tree1, tree2), true);
}
