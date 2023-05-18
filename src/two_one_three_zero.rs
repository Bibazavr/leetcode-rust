// https://leetcode.com/problems/maximum-twin-sum-of-a-linked-list/
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

impl Solution {
    pub fn pair_sum(mut head: Option<Box<ListNode>>) -> i32 {
        // get mid node
        let mut slow = &head;
        let mut fast = &head;
        // as far as it's even length, we can skip fast.next check
        while fast.is_some() {
            slow = &slow.as_ref().unwrap().next;
            fast = &fast.as_ref().unwrap().next.as_ref().unwrap().next;
        }
        // reverse second half
        let mut prev: Option<Box<ListNode>> = None;
        let mut cur = slow.clone();
        while let Some(mut node) = cur.take() {
            let next = node.next.take();
            node.next = prev.take();
            prev = Some(node);
            cur = next;
        }
        // traverse from both sides, pick max sum
        let mut back = prev;
        let mut max_sum = i32::MIN;
        while let Some(mut b_node) = back.take() {
            let mut f_node = head.unwrap();
            max_sum = max_sum.max(f_node.val + b_node.val);
            head = f_node.next.take();
            back = b_node.next.take();
        }
        max_sum
    }
}

#[allow(dead_code)]
pub fn main() {
    let head = Option::from(Box::new(ListNode {
        val: 5,
        next: Option::from(Box::new(ListNode {
            val: 4,
            next: Option::from(Box::new(ListNode {
                val: 2,
                next: Option::from(Box::new(ListNode { val: 1, next: None })),
            })),
        })),
    }));
    assert_eq!(Solution::pair_sum(head), 6);
}
