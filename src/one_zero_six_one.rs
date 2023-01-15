// https://leetcode.com/problems/lexicographically-smallest-equivalent-string/

struct Solution;

impl Solution {
    pub fn smallest_equivalent_string(s1: String, s2: String, base_str: String) -> String {
        let mut answer = String::with_capacity(base_str.len());
        let mut union_find_arr = vec![usize::MAX; 26];
        let s1 = s1.as_bytes();
        let s2 = s2.as_bytes();

        for i in 0..s1.len() {
            Solution::union(
                &mut union_find_arr,
                (s1[i] - b'a') as usize,
                (s2[i] - b'a') as usize,
            );
        }

        for char in base_str.bytes() {
            answer.push(
                (b'a' + Self::find(&mut union_find_arr, (char - b'a') as usize) as u8) as char,
            )
        }

        answer
    }

    fn find(union_find_arr: &mut Vec<usize>, a: usize) -> usize {
        if union_find_arr[a] == usize::MAX {
            union_find_arr[a] = a;
            return a;
        }

        let mut b = a;
        while union_find_arr[b] != b {
            b = union_find_arr[b];
        }
        union_find_arr[a] = b;
        b
    }

    fn union(union_find_arr: &mut Vec<usize>, a: usize, b: usize) {
        let a = Self::find(union_find_arr, a);
        let b = Self::find(union_find_arr, b);
        if a < b {
            union_find_arr[b] = a;
        } else {
            union_find_arr[a] = b;
        }
    }
}

#[allow(dead_code)]
pub fn main() {
    let s1 = String::from("parker");
    let s2 = String::from("morris");
    let base_str = String::from("parser");
    assert_eq!(
        Solution::smallest_equivalent_string(s1, s2, base_str),
        String::from("makkek")
    );
}
