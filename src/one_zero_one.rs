// https://leetcode.com/problems/symmetric-tree/

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
use std::collections::VecDeque;
use std::rc::Rc;
impl Solution {
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match root {
            None => true,
            Some(main) => {
                let main = RefCell::borrow(&main);

                let mut queue = VecDeque::new();
                queue.push_back((main.left.clone(), main.right.clone()));

                while let Some(next) = queue.pop_back() {
                    match next {
                        (Some(node_pointer_left), Some(node_pointer_right)) => {
                            let node_left = RefCell::borrow(&node_pointer_left);
                            let node_right = RefCell::borrow(&node_pointer_right);
                            if node_left.val != node_right.val {
                                return false;
                            }
                            queue.push_back((node_left.left.clone(), node_right.right.clone()));
                            queue.push_back((node_left.right.clone(), node_right.left.clone()));
                        }
                        (None, None) => (),
                        _ => return false,
                    }
                }
                true
            }
        }
    }
}

#[allow(dead_code)]
pub fn main() {
    let tree1 = Option::from(Rc::from(RefCell::from(TreeNode {
        val: 1,
        left: Option::from(Rc::from(RefCell::from(TreeNode {
            val: 2,
            left: Option::from(Rc::from(RefCell::from(TreeNode {
                val: 3,
                left: None,
                right: None,
            }))),
            right: Option::from(Rc::from(RefCell::from(TreeNode {
                val: 4,
                left: None,
                right: None,
            }))),
        }))),
        right: Option::from(Rc::from(RefCell::from(TreeNode {
            val: 2,
            left: Option::from(Rc::from(RefCell::from(TreeNode {
                val: 4,
                left: None,
                right: None,
            }))),
            right: Option::from(Rc::from(RefCell::from(TreeNode {
                val: 3,
                left: None,
                right: None,
            }))),
        }))),
    })));

    assert_eq!(Solution::is_symmetric(tree1), true);
}
