// https://leetcode.com/problems/swap-nodes-in-pairs/
struct Solution;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[allow(dead_code)]
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
impl Solution {
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match head {
            Some(mut node) => match node.next {
                Some(mut next_node) => {
                    node.next = Self::swap_pairs(next_node.next);
                    next_node.next = Some(node);
                    Some(next_node)
                }
                _ => Some(node),
            },
            _ => head,
        }
    }
}

#[allow(dead_code)]
pub fn main() {
    let head = Option::from(Box::new(ListNode {
        val: 1,
        next: Option::from(Box::new(ListNode {
            val: 2,
            next: Option::from(Box::new(ListNode {
                val: 3,
                next: Option::from(Box::new(ListNode {
                    val: 4,
                    next: Option::from(Box::new(ListNode { val: 5, next: None })),
                })),
            })),
        })),
    }));
    assert_eq!(
        Solution::swap_pairs(head),
        Option::from(Box::new(ListNode {
            val: 2,
            next: Option::from(Box::new(ListNode {
                val: 1,
                next: Option::from(Box::new(ListNode {
                    val: 4,
                    next: Option::from(Box::new(ListNode {
                        val: 3,
                        next: Option::from(Box::new(ListNode { val: 5, next: None })),
                    })),
                })),
            })),
        }))
    );
}
