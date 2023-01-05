use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut m: HashMap<i32, usize> = HashMap::new();

        for (idx1, value) in nums.iter().enumerate() {
            if let Some(idx2) = m.get(&value) {
                // We found the answer indicies
                return vec![idx1 as i32, *idx2 as i32];
            }
            // Otherwise, insert it to the hashmap
            m.insert(target - value, idx1);
        }
        // Since we can assume thare is exectly one solution,
        // this line should be unreachable.
        unreachable!();
    }
}

#[allow(dead_code)]
pub fn main() {
    let nums: Vec<i32> = vec![2, 5, 5, 11];
    let target = 10;
    let answer = Solution::two_sum(nums, target);
    println!("{:?}", answer)
}
