// https://leetcode.com/problems/find-duplicate-subtrees/

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
use std::collections::HashMap;
use std::rc::Rc;
impl Solution {
    pub fn find_duplicate_subtrees(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        let mut result = vec![];

        // Keep track of seen subtrees
        let mut seen = HashMap::new();

        Self::dfs(root, &mut seen, &mut result);

        result
    }

    pub fn dfs(
        root: Option<Rc<RefCell<TreeNode>>>,
        seen: &mut HashMap<String, u8>,
        result: &mut Vec<Option<Rc<RefCell<TreeNode>>>>,
    ) -> String {
        if let Some(node_pointer) = root {
            let node = RefCell::borrow(&node_pointer);
            let tree = format!(
                "{}l{}r{}",
                node.val,
                Self::dfs(node.left.clone(), seen, result),
                Self::dfs(node.right.clone(), seen, result)
            );

            let counter = seen.entry(tree.clone()).or_insert(0);
            *counter += 1;
            if *counter == 2 {
                result.push(Some(node_pointer.clone()));
            }

            tree
        } else {
            String::new()
        }
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
            right: None,
        }))),
        right: Option::from(Rc::from(RefCell::from(TreeNode {
            val: 3,
            left: Option::from(Rc::from(RefCell::from(TreeNode {
                val: 2,
                left: Option::from(Rc::from(RefCell::from(TreeNode {
                    val: 4,
                    left: None,
                    right: None,
                }))),
                right: None,
            }))),
            right: Option::from(Rc::from(RefCell::from(TreeNode {
                val: 4,
                left: None,
                right: None,
            }))),
        }))),
    })));
    assert_eq!(
        Solution::find_duplicate_subtrees(root),
        vec![
            Option::from(Rc::from(RefCell::from(TreeNode {
                val: 4,
                left: None,
                right: None,
            }))),
            Option::from(Rc::from(RefCell::from(TreeNode {
                val: 2,
                left: Option::from(Rc::from(RefCell::from(TreeNode {
                    val: 4,
                    left: None,
                    right: None,
                }))),
                right: None,
            }))),
        ]
    );
}
