// https://leetcode.com/problems/binary-tree-zigzag-level-order-traversal/

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
use std::collections::VecDeque;
use std::rc::Rc;
impl Solution {
    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        if root.is_none() {
            return Vec::new();
        }

        let mut result: Vec<Vec<i32>> = Vec::new();
        let mut lvl: u16 = 0;
        let mut queue: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();

        match root {
            Some(node_pointer) => {
                queue.push_back(node_pointer);
            }
            None => {}
        }

        while !queue.is_empty() {
            let len_q: usize = queue.len();
            let mut arr: Vec<i32> = Vec::new();
            for _ in 0..len_q {
                if let Some(node_pointer) = queue.pop_front() {
                    let node = node_pointer.borrow();
                    arr.push(node_pointer.borrow().val);
                    if let Some(l) = node.left.clone() {
                        queue.push_back(l);
                    }
                    if let Some(r) = node.right.clone() {
                        queue.push_back(r);
                    }
                }
            }
            if lvl % 2 == 1 {
                arr.reverse();
            }
            lvl += 1;
            result.push(arr);
        }
        result
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
    assert_eq!(
        Solution::zigzag_level_order(root),
        vec![vec![3], vec![20, 9], vec![15, 7]]
    );
}
