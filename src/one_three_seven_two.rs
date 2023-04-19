// https://leetcode.com/problems/longest-zigzag-path-in-a-binary-tree/

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
    pub fn longest_zig_zag(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, l_count: i32, r_count: i32) -> i32 {
            match node.as_ref() {
                None => l_count.max(r_count) - 1,
                Some(n) => {
                    let b = n.borrow();
                    dfs(&b.left, r_count + 1, 0).max(dfs(&b.right, 0, l_count + 1))
                }
            }
        }
        dfs(&root, 0, 0)
    }
}

#[allow(dead_code)]
pub fn main() {
    let root = Option::from(Rc::from(RefCell::from(TreeNode {
        val: 1,
        left: None,
        right: Option::from(Rc::from(RefCell::from(TreeNode {
            val: 1,
            left: Option::from(Rc::from(RefCell::from(TreeNode {
                val: 2,
                left: None,
                right: None,
            }))),
            right: Option::from(Rc::from(RefCell::from(TreeNode {
                val: 2,
                left: Option::from(Rc::from(RefCell::from(TreeNode {
                    val: 2,
                    left: None,
                    right: Option::from(Rc::from(RefCell::from(TreeNode {
                        val: 2,
                        left: None,
                        right: Option::from(Rc::from(RefCell::from(TreeNode {
                            val: 2,
                            left: None,
                            right: None,
                        }))),
                    }))),
                }))),
                right: Option::from(Rc::from(RefCell::from(TreeNode {
                    val: 2,
                    left: None,
                    right: None,
                }))),
            }))),
        }))),
    })));
    assert_eq!(Solution::longest_zig_zag(root), 3)
}
