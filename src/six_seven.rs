// https://leetcode.com/problems/add-binary/

struct Solution;

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let a_u = u128::from_str_radix(&a, 2);
        let b_u = u128::from_str_radix(&b, 2);

        return match (a_u, b_u) {
            (Ok(a), Ok(b)) => {
                format!("{:b}", a + b)
            }
            (_, _) => String::new(),
        };
    }
}

#[allow(dead_code)]
pub fn main() {
    let a = String::from("11");
    let b = String::from("1");
    assert_eq!(Solution::add_binary(a, b), String::from("100"));
}
