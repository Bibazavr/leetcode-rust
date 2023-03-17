//https://leetcode.com/problems/implement-trie-prefix-tree/

use std::collections::HashMap;

struct TrieNode {
    children: HashMap<char, TrieNode>,
    is_word: bool,
}

impl TrieNode {
    fn new() -> Self {
        TrieNode {
            children: HashMap::new(),
            is_word: false,
        }
    }
}

struct Trie {
    root: TrieNode,
}

/**
 * `&self` means the method takes an immutable reference
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Trie {
    fn new() -> Self {
        Trie {
            root: TrieNode::new(),
        }
    }

    fn insert(&mut self, word: String) {
        let mut current_node = &mut self.root;

        for c in word.chars() {
            let next_node = current_node.children.entry(c).or_insert(TrieNode::new());
            current_node = next_node;
        }
        current_node.is_word = true;
    }

    fn search(&self, word: String) -> bool {
        let mut current_node = &self.root;

        for c in word.chars() {
            match current_node.children.get(&c) {
                Some(next_node) => current_node = next_node,
                None => return false,
            }
        }

        return current_node.is_word;
    }

    fn starts_with(&self, prefix: String) -> bool {
        let mut current_node = &self.root;

        for c in prefix.chars() {
            match current_node.children.get(&c) {
                Some(next_node) => current_node = next_node,
                None => return false,
            }
        }

        return true;
    }
}

/**
 * Your Trie object will be instantiated and called as such:
 * let obj = Trie::new();
 * obj.insert(word);
 * let ret_2: bool = obj.search(word);
 * let ret_3: bool = obj.starts_with(prefix);
 */

#[allow(dead_code)]
pub fn main() {
    let mut trie = Trie::new();
    trie.insert(String::from("apple"));
    assert_eq!(trie.search(String::from("apple")), true);
    assert_eq!(trie.search(String::from("app")), false);
    assert_eq!(trie.starts_with(String::from("app")), true);
    trie.insert(String::from("app"));
    assert_eq!(trie.search(String::from("app")), true);
}
