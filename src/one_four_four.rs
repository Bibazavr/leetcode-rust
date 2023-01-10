// https://leetcode.com/problems/binary-tree-preorder-traversal/

struct Solution;

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
    pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = vec![];
        let mut stack = vec![root];
        while let Some(node_opt) = stack.pop() {
            if let Some(good_node) = node_opt {
                let node = good_node.borrow();
                result.push(node.val);
                stack.push(node.right.clone());
                stack.push(node.left.clone());
            }
        }
        result
    }
    fn preorder_traversal_recurs(root: Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<i32>) {
        if let Some(pnode) = root {
            let node = pnode.borrow();
            result.push(node.val);
            Self::preorder_traversal_recurs(node.left.clone(), result);
            Self::preorder_traversal_recurs(node.right.clone(), result);
        }
    }
}

#[allow(dead_code)]
pub fn main() {
    let lists = Option::from(Rc::from(RefCell::from(TreeNode {
        val: 1,
        left: None,
        right: None,
    })));

    let answer = Solution::preorder_traversal(lists);
    assert_eq!(answer, vec![1]);
}
