// https://leetcode.com/problems/convert-sorted-list-to-binary-search-tree/

struct Solution;
// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    #[allow(dead_code)]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
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
    // More simple Solution
    // pub fn sorted_list_to_bst(mut head: Option<Box<ListNode>>) -> Node {
    //     let mut nums = Vec::new();
    //     while let Some(node) = head {
    //         nums.push(node.val);
    //         head = node.next;
    //     }
    //     Self::binarySearch(&nums)
    // }
    // fn binarySearch(nums: &[i32]) -> Node {
    //     let mut n = nums.len();
    //     match n {
    //         0 => None,
    //         _ => {
    //             let mid = n / 2;
    //             let mut root = TreeNode::new(nums[mid]);
    //             root.left = Self::binarySearch(&nums[..mid]);
    //             root.right = Self::binarySearch(&nums[mid + 1..]);
    //             Some(Rc::new(RefCell::new(root)))
    //         }
    //     }
    // }

    pub fn sorted_list_to_bst(mut head: Option<Box<ListNode>>) -> Option<Rc<RefCell<TreeNode>>> {
        let length = Self::list_length(head.as_ref());
        Self::list_to_bst(&mut head, 0, length)
    }

    pub fn list_to_bst(
        head: &mut Option<Box<ListNode>>,
        start: usize,
        end: usize,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if start >= end {
            return None;
        }

        let mid = start + (end - start) / 2;

        let left = Self::list_to_bst(head, start, mid);

        let mut current = head.take().unwrap();
        *head = current.next.take();

        let right = Self::list_to_bst(head, mid + 1, end);

        Some(Rc::new(RefCell::new(TreeNode {
            val: current.val,
            left,
            right,
        })))
    }

    fn list_length(head: Option<&Box<ListNode>>) -> usize {
        let mut node = head;
        let mut length = 0;

        while let Some(n) = node {
            node = n.next.as_ref();
            length += 1;
        }

        length
    }
}
#[allow(dead_code)]
pub fn main() {
    let root = Option::from(Box::new(ListNode {
        val: -10,
        next: Option::from(Box::new(ListNode {
            val: -3,
            next: Option::from(Box::new(ListNode {
                val: 0,
                next: Option::from(Box::new(ListNode {
                    val: 5,
                    next: Option::from(Box::new(ListNode { val: 9, next: None })),
                })),
            })),
        })),
    }));

    assert_eq!(
        Solution::sorted_list_to_bst(root),
        Option::from(Rc::from(RefCell::from(TreeNode {
            val: 0,
            left: Option::from(Rc::from(RefCell::from(TreeNode {
                val: -3,
                left: Option::from(Rc::from(RefCell::from(TreeNode {
                    val: -10,
                    left: None,
                    right: None,
                }))),
                right: None,
            }))),
            right: Option::from(Rc::from(RefCell::from(TreeNode {
                val: 9,
                left: Option::from(Rc::from(RefCell::from(TreeNode {
                    val: 5,
                    left: None,
                    right: None,
                }))),
                right: None,
            }))),
        }))),
    )
}
