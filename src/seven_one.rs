// https://leetcode.com/problems/simplify-path/

struct Solution;

impl Solution {
    pub fn simplify_path(path: String) -> String {
        let mut simplified_path = vec![];
        for dir in path.split('/') {
            match dir {
                "" | "." => continue,
                ".." => {
                    simplified_path.pop();
                }
                _ => simplified_path.push(dir),
            }
        }

        "/".to_owned() + &simplified_path.join("/")
    }
}

#[allow(dead_code)]
pub fn main() {
    let path = String::from("/home/");
    assert_eq!(Solution::simplify_path(path), String::from("/home"));
}
