// https://leetcode.com/problems/smallest-number-in-infinite-set/

use std::collections::{BinaryHeap, HashSet};

struct SmallestInfiniteSet {
    heap: BinaryHeap<i32>,
    set: HashSet<i32>,
    smallest: i32,
}

/**
 * `&self` means the method takes an immutable reference
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl SmallestInfiniteSet {
    fn new() -> Self {
        SmallestInfiniteSet {
            heap: BinaryHeap::new(),
            set: HashSet::new(),
            smallest: 1,
        }
    }

    fn pop_smallest(&mut self) -> i32 {
        if self.heap.is_empty() {
            self.smallest += 1;
            self.smallest - 1
        } else {
            let smallest = self.heap.pop().unwrap();
            self.set.remove(&smallest);
            -smallest
        }
    }

    fn add_back(&mut self, num: i32) {
        if num >= self.smallest {
            return;
        }
        if !self.set.contains(&(-num)) {
            self.heap.push(-num);
            self.set.insert(-num);
        }
    }
}

/**
 * Your SmallestInfiniteSet object will be instantiated and called as such:
 * let obj = SmallestInfiniteSet::new();
 * let ret_1: i32 = obj.pop_smallest();
 * obj.add_back(num);
 */

#[allow(dead_code)]
pub fn main() {
    let mut smallest_infinite_set = SmallestInfiniteSet::new();
    smallest_infinite_set.add_back(2); // 2 is already in the set, so no change is made.
    smallest_infinite_set.pop_smallest(); // return 1, since 1 is the smallest number, and remove it from the set.
    smallest_infinite_set.pop_smallest(); // return 2, and remove it from the set.
    smallest_infinite_set.pop_smallest(); // return 3, and remove it from the set.
    smallest_infinite_set.add_back(1); // 1 is added back to the set.
    smallest_infinite_set.pop_smallest(); // return 1, since 1 was added back to the set and
                                          // is the smallest number, and remove it from the set.
    smallest_infinite_set.pop_smallest(); // return 4, and remove it from the set.
    smallest_infinite_set.pop_smallest(); // return 5, and remove it from the set.
}
