// https://leetcode.com/problems/edit-distance/

struct Solution;

impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let (word1_bytes, word2_bytes) = (word1.as_bytes(), word2.as_bytes());

        let mut dist = Vec::with_capacity(word2_bytes.len() + 1);

        for j in 0..=word2_bytes.len() {
            dist.push(j)
        }

        let mut prev_dist = dist.clone();

        for i in 1..=word1_bytes.len() {
            for j in 0..=word2_bytes.len() {
                if j == 0 {
                    dist[j] += 1;
                } else if word1_bytes[i - 1] == word2_bytes[j - 1] {
                    dist[j] = prev_dist[j - 1];
                } else {
                    dist[j] = dist[j].min(dist[j - 1]).min(prev_dist[j - 1]) + 1;
                }
            }
            prev_dist.copy_from_slice(&dist);
        }
        dist[word2_bytes.len()] as i32
    }
}

#[allow(dead_code)]
pub fn main() {
    let a = String::from("horse");
    let b = String::from("ros");
    assert_eq!(Solution::min_distance(a, b), 3);
}
