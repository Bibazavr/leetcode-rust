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

struct Solution;

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match (l1, l2) {
            (None, None) => None,
            ((Some(n), None) | (None, Some(n))) => Some(n),
            (Some(n1), Some(n2)) => {
                let sum = n1.val + n2.val;
                if sum >= 10 {
                    let carry = Some(Box::new(ListNode::new(1)));
                    Some(Box::new(ListNode {
                        val: sum - 10,
                        next: Solution::add_two_numbers(
                            n1.next,
                            Solution::add_two_numbers(n2.next, carry),
                        ),
                    }))
                } else {
                    Some(Box::new(ListNode {
                        val: sum,
                        next: Solution::add_two_numbers(n1.next, n2.next),
                    }))
                }
            }
        }
    }
}

pub fn main() {
    let l1: Option<Box<ListNode>> = Option::from(Box::new(ListNode {
        val: 2,
        next: Option::from(Box::new(ListNode {
            val: 4,
            next: Option::from(Box::new(ListNode { val: 3, next: None })),
        })),
    }));
    let l2 = Option::from(Box::new(ListNode {
        val: 5,
        next: Option::from(Box::new(ListNode {
            val: 6,
            next: Option::from(Box::new(ListNode { val: 4, next: None })),
        })),
    }));

    let answer = Solution::add_two_numbers(l1, l2);
    println!("{:?}", answer)
}
