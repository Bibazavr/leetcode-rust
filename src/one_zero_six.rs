// https://leetcode.com/problems/construct-binary-tree-from-inorder-and-postorder-traversal/

struct Solution;

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
use std::collections::HashMap;
use std::rc::Rc;
impl Solution {
    pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let hash: HashMap<i32, usize> = inorder.iter().enumerate().map(|(i, &x)| (x, i)).collect();
        Self::build_node(&postorder, &hash, 0)
    }
    fn build_node(
        postorder: &[i32],
        hash: &HashMap<i32, usize>,
        offset: usize,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if postorder.is_empty() {
            return None;
        }
        let n = postorder.len() - 1;
        let pos = hash.get(&postorder[n]).unwrap() - offset;
        Some(Rc::new(RefCell::new(TreeNode {
            val: postorder[n],
            left: Self::build_node(&postorder[..pos], hash, offset),
            right: Self::build_node(&postorder[pos..n], hash, offset + pos + 1),
        })))
    }
}

#[allow(dead_code)]
pub fn main() {
    let inorder = vec![9, 3, 15, 20, 7];
    let postorder = vec![9, 15, 7, 20, 3];
    assert_eq!(
        Solution::build_tree(inorder, postorder),
        Option::from(Rc::from(RefCell::from(TreeNode {
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
        })))
    );
}
