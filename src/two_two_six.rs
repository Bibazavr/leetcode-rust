// https://leetcode.com/problems/invert-binary-tree/

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
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        match root {
            Some(root_pointer) => {
                let node = root_pointer.borrow();
                Option::from(Rc::from(RefCell::from(TreeNode {
                    val: node.val,
                    left: Self::invert_tree(node.right.clone()),
                    right: Self::invert_tree(node.left.clone()),
                })))
            }
            None => root,
        }
    }
}

#[allow(dead_code)]
pub fn main() {
    let root = Option::from(Rc::from(RefCell::from(TreeNode {
        val: 1,
        left: None,
        right: None,
    })));

    assert_eq!(
        Solution::invert_tree(root),
        Option::from(Rc::from(RefCell::from(TreeNode {
            val: 1,
            left: None,
            right: None,
        })))
    );
}
