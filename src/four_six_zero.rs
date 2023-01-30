// https://leetcode.com/problems/lfu-cache/

use std::collections::{btree_map::Entry, BTreeMap, BTreeSet};

struct Node {
    frequency: i32,
    call_counter: u32,
    value: i32,
}

pub struct LFUCache {
    capacity: usize,
    call_counter: u32,
    nodes: BTreeMap<i32, Node>,
    hash: BTreeSet<(i32, u32, i32)>,
}

impl LFUCache {
    pub fn new(capacity: i32) -> Self {
        Self {
            capacity: capacity as usize,
            call_counter: 1,
            nodes: BTreeMap::new(),
            hash: BTreeSet::new(),
        }
    }

    pub fn get(&mut self, key: i32) -> i32 {
        self.call_counter += 1;
        if let Entry::Occupied(mut node) = self.nodes.entry(key) {
            let mut node_mut = node.get_mut();

            self.hash
                .take(&(node_mut.frequency, node_mut.call_counter, key));

            node_mut.frequency += 1;
            node_mut.call_counter = self.call_counter;

            self.hash
                .insert((node_mut.frequency, node_mut.call_counter, key));

            return node.get().value;
        }

        -1
    }

    pub fn put(&mut self, key: i32, value: i32) {
        self.call_counter += 1;

        if let Entry::Occupied(mut node) = self.nodes.entry(key) {
            let mut node_mut = node.get_mut();

            self.hash
                .take(&(node_mut.frequency, node_mut.call_counter, key));

            node_mut.value = value;
            node_mut.call_counter = self.call_counter;
            node_mut.frequency += 1;

            self.hash
                .insert((node_mut.frequency, node_mut.call_counter, key));
        } else if self.capacity > 0 {
            let new_node = Node {
                value,
                frequency: 1,
                call_counter: self.call_counter,
            };

            if self.nodes.len() >= self.capacity {
                // let d = self.hash.pop_first().unwrap();
                let d = self.hash.iter().next().unwrap().clone();
                self.hash.remove(&d);
                self.nodes.remove(&d.2);
            }

            self.hash
                .insert((new_node.frequency, new_node.call_counter, key));
            self.nodes.insert(key, new_node);
        }
    }
}

/**
 * Your LFUCache object will be instantiated and called as such:
 * let obj = LFUCache::new(capacity);
 * let ret_1: i32 = obj.get(key);
 * obj.put(key, value);
["LFUCache","put","put","get","put","get","get","put","get","get","get"]
[[2],[1,1],[2,2],[1],[3,3],[2],[3],[4,4],[1],[3],[4]]
 */

#[allow(dead_code)]
pub fn main() {
    let mut cache = LFUCache::new(2);
    cache.put(1, 1);
    cache.put(2, 2);
    assert_eq!(cache.get(1), 1);

    cache.put(3, 3);
    assert_eq!(cache.get(2), -1);
    assert_eq!(cache.get(3), 3);

    cache.put(4, 4);
    assert_eq!(cache.get(1), -1);
    assert_eq!(cache.get(3), 3);
    assert_eq!(cache.get(4), 4);

    //["LFUCache","put","put","get","get","put","get","get","get"]
    //[[2],[2,1],[3,2],[3],[2],[4,3],[2],[3],[4]]
    let mut lfu = LFUCache::new(2);
    lfu.put(2, 1);
    lfu.put(3, 2);
    assert_eq!(lfu.get(3), 2);

    assert_eq!(lfu.get(2), 1);

    lfu.put(4, 3);
    assert_eq!(lfu.get(2), 1);
    assert_eq!(lfu.get(3), -1);
    assert_eq!(lfu.get(4), 3);
}
