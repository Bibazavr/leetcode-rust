// https://leetcode.com/problems/minimum-distance-between-bst-nodes/

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
    pub fn min_diff_in_bst(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut arr: Vec<i32> = vec![];

        Self::fill_arr(root, &mut arr);

        arr.windows(2)
            .fold(i32::MAX, |acc, vals| acc.min(vals[1] - vals[0]))
    }

    pub fn fill_arr(root: Option<Rc<RefCell<TreeNode>>>, arr: &mut Vec<i32>) {
        if let Some(node_pointer) = root {
            let node = node_pointer.borrow();
            Self::fill_arr(node.left.clone(), arr);
            arr.push(node.val);
            Self::fill_arr(node.right.clone(), arr);
        }
    }
}

#[allow(dead_code)]
pub fn main() {
    let root = Option::from(Rc::from(RefCell::from(TreeNode {
        val: 4,
        left: Option::from(Rc::from(RefCell::from(TreeNode {
            val: 2,
            left: Option::from(Rc::from(RefCell::from(TreeNode {
                val: 1,
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
            val: 6,
            left: None,
            right: None,
        }))),
    })));
    assert_eq!(Solution::min_diff_in_bst(root), 1);
}
