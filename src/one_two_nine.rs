// https://leetcode.com/problems/sum-root-to-leaf-numbers/

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
    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::sum(root, 0)
    }

    pub fn sum(root: Option<Rc<RefCell<TreeNode>>>, current: i32) -> i32 {
        match root {
            None => 0,
            Some(node_pointer) => {
                let node = RefCell::borrow(&node_pointer);

                let new_current = current * 10 + node.val;

                let left = node.left.clone();
                let right = node.right.clone();

                match (&left, &right) {
                    (None, None) => new_current,
                    _ => Solution::sum(left, new_current) + Solution::sum(right, new_current),
                }
            }
        }
    }
}

#[allow(dead_code)]
pub fn main() {
    let root = Option::from(Rc::from(RefCell::from(TreeNode {
        val: 1,
        left: Option::from(Rc::from(RefCell::from(TreeNode {
            val: 0,
            left: None,
            right: None,
        }))),
        right: None,
    })));
    assert_eq!(Solution::sum_numbers(root), 10);

    let root = Option::from(Rc::from(RefCell::from(TreeNode {
        val: 1,
        left: Option::from(Rc::from(RefCell::from(TreeNode {
            val: 2,
            left: None,
            right: None,
        }))),
        right: Option::from(Rc::from(RefCell::from(TreeNode {
            val: 3,
            left: None,
            right: None,
        }))),
    })));
    assert_eq!(Solution::sum_numbers(root), 25)
}
