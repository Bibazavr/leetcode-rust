// https://leetcode.com/problems/linked-list-random-node/description/

const MULTIPLIER: u64 = 6364136223846793005;
const INCREMENT: u64 = 1442695040888963407;
use std::time::SystemTime;
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
struct Solution {
    head: Option<Box<ListNode>>,
    seed: u64,
}

/**
 * `&self` means the method takes an immutable reference
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {
    fn new(head: Option<Box<ListNode>>) -> Self {
        let seed = SystemTime::now().elapsed().unwrap().as_nanos() as u64;
        Self { head, seed }
    }

    fn get_random(&mut self) -> i32 {
        let mut seed = self.seed;

        let mut quickrand = || {
            seed = seed.wrapping_mul(MULTIPLIER).wrapping_add(INCREMENT);
            seed as f64 / u64::MAX as f64
        };

        let mut node_opt = self.head.as_ref();
        let mut rand_val = node_opt.unwrap().val;
        let mut i = 1.0;

        while let Some(node) = node_opt {
            if quickrand() < 1.0 / i {
                rand_val = node.val;
            }
            i += 1.0;
            node_opt = node.next.as_ref();
        }
        self.seed = seed;
        rand_val
    }
}

/**
 * Your Solution object will be instantiated and called as such:
 * let obj = Solution::new(head);
 * let ret_1: i32 = obj.get_random();
 */

#[allow(dead_code)]
pub fn main() {
    let head = Option::from(Box::new(ListNode {
        val: 1,
        next: Option::from(Box::new(ListNode {
            val: 2,
            next: Option::from(Box::new(ListNode { val: 3, next: None })),
        })),
    }));
    let mut obj = Solution::new(head);
    assert_eq!(obj.get_random(), 1);
    assert_eq!(obj.get_random(), 3);
    assert_eq!(obj.get_random(), 2);
    assert_eq!(obj.get_random(), 2);
    assert_eq!(obj.get_random(), 3);
}
