// https://leetcode.com/problems/design-add-and-search-words-data-structure/

#[derive(Debug, Clone)]
struct TrieNode {
    links: Vec<Option<TrieNode>>,
    is_end: bool,
}
impl TrieNode {
    fn new() -> Self {
        Self {
            links: vec![None; 26],
            is_end: false,
        }
    }
    fn contains(&self, key: char) -> bool {
        self.links[key as usize - 'a' as usize].is_some()
    }
    fn put(&mut self, key: char, value: TrieNode) {
        self.links[key as usize - 'a' as usize] = Some(value);
    }
    fn get(&self, key: char) -> Option<&TrieNode> {
        self.links[key as usize - 'a' as usize].as_ref()
    }
    fn get_mut(&mut self, key: char) -> Option<&mut TrieNode> {
        self.links[key as usize - 'a' as usize].as_mut()
    }
}

struct Trie {
    root: TrieNode,
}
impl Trie {
    fn new() -> Self {
        Self {
            root: TrieNode::new(),
        }
    }
    fn add_word(&mut self, word: String) {
        let mut root = &mut self.root;
        for ch in word.chars() {
            if !root.contains(ch) {
                root.put(ch, TrieNode::new());
            }
            root = root.get_mut(ch).unwrap();
        }
        root.is_end = true;
    }
}

struct WordDictionary {
    trie: Trie,
}

/**
 * `&self` means the method takes an immutable reference
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl WordDictionary {
    fn new() -> Self {
        Self { trie: Trie::new() }
    }

    fn add_word(&mut self, word: String) {
        self.trie.add_word(word)
    }

    fn search(&self, word: String) -> bool {
        let mut found = false;

        let word_bytes = word.chars().collect::<Vec<_>>();

        let mut stack = vec![];
        stack.push((&self.trie.root, 0usize));

        while let Some((node, word_idx)) = stack.pop() {
            let cur_char = word_bytes[word_idx];

            if word_idx == word.len() - 1 {
                if cur_char == '.' {
                    found |= node.links.iter().flatten().filter(|x| x.is_end).count() > 0;
                } else if let Some(trie_node) = node.get(cur_char) {
                    found |= trie_node.is_end;
                }
                continue;
            }

            if cur_char == '.' {
                node.links
                    .iter()
                    .flatten()
                    .for_each(|x| stack.push((x, word_idx + 1)));
            } else if let Some(trie_node) = node.get(cur_char) {
                stack.push((trie_node, word_idx + 1));
            }
        }

        found
    }
}

/**
 * Your WordDictionary object will be instantiated and called as such:
 * let obj = WordDictionary::new();
 * obj.add_word(word);
 * let ret_2: bool = obj.search(word);
 */
#[allow(dead_code)]
pub fn main() {
    let mut word_dictionary = WordDictionary::new();
    word_dictionary.add_word(String::from("bad"));
    word_dictionary.add_word(String::from("dad"));
    word_dictionary.add_word(String::from("mad"));
    assert_eq!(word_dictionary.search(String::from("pad")), false); // return False
    assert_eq!(word_dictionary.search(String::from("bad")), true); // return True
    assert_eq!(word_dictionary.search(String::from(".ad")), true); // return True
    assert_eq!(word_dictionary.search(String::from("b..")), true); // return True
}
