// https://leetcode.com/problems/verifying-an-alien-dictionary/

struct Solution;

impl Solution {
    pub fn is_alien_sorted(words: Vec<String>, order: String) -> bool {
        let char_indices = order
            .char_indices()
            .map(|(i, c)| (c, i))
            .collect::<std::collections::HashMap<char, usize>>();

        words.windows(2).all(|ws| {
            for (c1, c2) in ws[0].chars().zip(ws[1].chars()) {
                let l1 = char_indices.get(&c1).unwrap();
                let l2 = char_indices.get(&c2).unwrap();
                if l1 < l2 {
                    return true;
                } else if l1 > l2 {
                    return false;
                }
            }
            ws[0].len() <= ws[1].len()
        })
    }
}

#[allow(dead_code)]
pub fn main() {
    let words = vec![String::from("hello"), String::from("leetcode")];
    let order = String::from("worldabcefghijkmnpqstuvxyz");
    assert_eq!(Solution::is_alien_sorted(words, order), true);

    let words = vec![
        String::from("word"),
        String::from("world"),
        String::from("row"),
    ];
    let order = String::from("hlabcdefgijkmnopqrstuvwxyz");
    assert_eq!(Solution::is_alien_sorted(words, order), false);
}
