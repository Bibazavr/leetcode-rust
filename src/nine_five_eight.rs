// https://leetcode.com/problems/check-completeness-of-a-binary-tree/

struct Solution;
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
    pub fn is_complete_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(root);

        let mut is_previous_none = false;

        while let Some(next) = queue.pop_front() {
            match next {
                None => {
                    is_previous_none = true;
                    continue;
                }
                Some(node_pointer) => {
                    if is_previous_none {
                        return false;
                    }

                    let node = RefCell::borrow(&node_pointer);
                    queue.push_back(node.left.clone());
                    queue.push_back(node.right.clone());
                }
            }
        }

        true
    }
}

#[allow(dead_code)]
pub fn main() {
    let root = Option::from(Rc::from(RefCell::from(TreeNode {
        val: 1,
        left: Option::from(Rc::from(RefCell::from(TreeNode {
            val: 2,
            left: Option::from(Rc::from(RefCell::from(TreeNode {
                val: 4,
                left: None,
                right: None,
            }))),
            right: Option::from(Rc::from(RefCell::from(TreeNode {
                val: 5,
                left: None,
                right: None,
            }))),
        }))),
        right: Option::from(Rc::from(RefCell::from(TreeNode {
            val: 3,
            left: Option::from(Rc::from(RefCell::from(TreeNode {
                val: 6,
                left: None,
                right: None,
            }))),
            right: None,
        }))),
    })));

    assert_eq!(Solution::is_complete_tree(root), true);
}
