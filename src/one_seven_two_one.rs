// https://leetcode.com/problems/swapping-nodes-in-a-linked-list/
struct Solution;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
impl Solution {
    pub fn swap_nodes(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut len = 0;
        let mut curr = &head;
        let mut from_head = 0;

        while let Some(node) = curr {
            if len + 1 == k {
                from_head = node.val;
            }
            curr = &node.next;
            len += 1;
        }

        let mut curr = &mut head;
        let mut i = 0;
        let mut from_tail = 0;

        while let Some(node) = curr {
            if i + k == len {
                from_tail = node.val;
                node.val = from_head;
            }
            curr = &mut node.next;
            i += 1;
        }

        let mut curr = &mut head;
        i = 0;

        while let Some(node) = curr {
            if i + 1 == k {
                node.val = from_tail;
                return head;
            }
            curr = &mut node.next;
            i += 1;
        }
        return head;
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
    let k = 2;
    assert_eq!(
        Solution::swap_nodes(head, k),
        Option::from(Box::new(ListNode {
            val: 1,
            next: Option::from(Box::new(ListNode {
                val: 4,
                next: Option::from(Box::new(ListNode {
                    val: 3,
                    next: Option::from(Box::new(ListNode {
                        val: 2,
                        next: Option::from(Box::new(ListNode { val: 5, next: None })),
                    })),
                })),
            })),
        }))
    );
}
