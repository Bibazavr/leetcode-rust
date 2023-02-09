// https://leetcode.com/problems/naming-a-company/
struct Solution;

impl Solution {
    pub fn distinct_names(ideas: Vec<String>) -> i64 {
        let initial_group = ideas.iter().fold(
            vec![std::collections::HashSet::new(); 26],
            |mut initial_group, idea| {
                initial_group[idea.as_bytes()[0] as usize - 'a' as usize].insert(&idea[1..]);
                initial_group
            },
        );

        (0..25).fold(0, |answer, i| {
            (i + 1..26).fold(answer, |answer, j| {
                let num_of_mutual = initial_group[i].iter().fold(0, |num_of_mutual, idea_a| {
                    match initial_group[j].contains(idea_a) {
                        true => num_of_mutual + 1,
                        false => num_of_mutual,
                    }
                });
                answer
                    + 2 * (initial_group[i].len() as i64 - num_of_mutual)
                        * (initial_group[j].len() as i64 - num_of_mutual)
            })
        })

        // first iteration - works but long
        // let mut hash = HashSet::new();
        // let mut result = 0;
        //
        // ideas.iter().for_each(|idea| {
        //     hash.insert(idea);
        // });
        //
        // for (i, &first) in hash.iter().enumerate() {
        //     let first_letter = first.chars().nth(0).unwrap();
        //     let first_off = first.chars().skip(1).collect::<String>();
        //
        //     for &second in hash.iter().skip(i + 1) {
        //         let second_letter = second.chars().nth(0).unwrap();
        //         let second_off = second.chars().skip(1).collect::<String>();
        //
        //         let first_swapped = String::from(format!("{second_letter}{first_off}"));
        //         let second_swapped = String::from(format!("{first_letter}{second_off}"));
        //
        //         println!("{} {}", first_swapped, second_swapped);
        //         if hash.contains(&first_swapped) || hash.contains(&second_swapped) {
        //             continue;
        //         }
        //         result += 1
        //     }
        // }
        //
        // result * 2
    }
}

#[allow(dead_code)]
pub fn main() {
    let ideas = vec![
        "coffee".to_string(),
        "donuts".to_string(),
        "time".to_string(),
        "toffee".to_string(),
    ];
    assert_eq!(Solution::distinct_names(ideas), 6);

    let ideas = vec!["lack".to_string(), "back".to_string()];
    assert_eq!(Solution::distinct_names(ideas), 0);
}
