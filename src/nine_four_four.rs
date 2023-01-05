struct Solution;

impl Solution {
    // not optimal but enough for me (first solution)
    pub fn min_deletion_size(strs: Vec<String>) -> i32 {
        (0..strs.len())
            .filter(|i| {
                strs.iter()
                    .map(|s| s.as_bytes()[*i])
                    .collect::<Vec<u8>>()
                    .windows(2)
                    .all(|parts| parts[0] <= parts[1])
                    == false
            })
            .count() as _
    }
}

#[allow(dead_code)]
pub fn main() {
    let case = vec!["qwe".to_string(), "ewq".to_string()];

    let answer = Solution::min_deletion_size(case);
    println!("{}", answer.to_string())
}
