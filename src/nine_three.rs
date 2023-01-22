// https://leetcode.com/problems/restore-ip-addresses/

struct Solution;

impl Solution {
    pub fn restore_ip_addresses(s: String) -> Vec<String> {
        let mut res = vec![];

        Self::back_track(&s.chars().collect(), &mut res, 0, &vec![]);

        return res;
    }

    fn back_track(s: &Vec<char>, res: &mut Vec<String>, i: usize, ip: &Vec<i32>) {
        if i == s.len() {
            if ip.len() == 4 {
                let mut new_ip = String::new();
                for (j, &num) in ip.into_iter().enumerate() {
                    let mut part = num.to_string();

                    if j != ip.len() - 1 {
                        part.push('.')
                    }

                    new_ip.push_str(&part)
                }
                res.push(new_ip)
            }

            return;
        }

        let mut clone = ip.clone();
        match clone.last() {
            Some(last_num) => {
                if last_num != &0 {
                    let was = s[i];
                    let new_num: i32 = last_num * 10 + was.to_string().parse::<i32>().unwrap();
                    if new_num <= 255 {
                        let n = clone.len();
                        clone[n - 1] = new_num.to_string().parse().unwrap();
                        Self::back_track(s, res, i + 1, &clone);
                        clone[n - 1] = was.to_string().parse::<i32>().unwrap();
                    }
                }
            }
            _ => {}
        }

        if ip.len() < 4 {
            let mut new_ip = ip.clone();
            let new_num = s[i].to_string().parse().unwrap();
            new_ip.push(new_num);
            Self::back_track(s, res, i + 1, &new_ip)
        }
    }
}

#[allow(dead_code)]
pub fn main() {
    let s = String::from("25525511135");
    assert_eq!(
        Solution::restore_ip_addresses(s),
        vec!["255.255.11.135", "255.255.111.35"]
    );
}
