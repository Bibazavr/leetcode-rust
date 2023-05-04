// https://leetcode.com/problems/dota2-senate/

struct Solution;

use std::collections::VecDeque;
use std::iter::FromIterator;

#[derive(PartialEq)]
enum Senate {
    R,
    D,
}
impl Solution {
    pub fn predict_party_victory(senate: String) -> String {
        senate.chars().collect::<DotaSenate>().get_winner()
    }
}

// Default is an empty Senate.
#[derive(Default)]
struct DotaSenate {
    vote_queue: VecDeque<Senate>,
    ban_queue: VecDeque<Senate>,
    radiant: usize,
    dire: usize,
}

impl DotaSenate {
    // Adds one voter to the end of the vote queue.
    pub fn add(&mut self, voter: char) {
        match voter {
            'R' => {
                self.vote_queue.push_back(Senate::R);
                self.radiant += 1;
            }
            'D' => {
                self.vote_queue.push_back(Senate::D);
                self.dire += 1;
            }
            _ => {}
        }
    }

    // Runs the iterator until a winner is found.
    pub fn get_winner(&mut self) -> String {
        self.find_map(|maybe_winner| maybe_winner).unwrap()
    }
}

// Lets us use .collect<DotaSenate>, which is cleaner.
impl FromIterator<char> for DotaSenate {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = char>,
    {
        let mut dota_senate = DotaSenate::default();
        iter.into_iter().for_each(|ch| dota_senate.add(ch));
        dota_senate
    }
}

// Each call to next() is a single voter taking an action.
// If the election is ongoing, will return Some(None).
impl Iterator for DotaSenate {
    type Item = Option<String>;

    fn next(&mut self) -> Option<Option<String>> {
        // If there are no voters, the iterator ends (returns None).
        let voter = self.vote_queue.pop_front()?;

        // Check if this voter has been banned
        if Some(&voter) == self.ban_queue.front() {
            // They have been banned, decrement voter records
            self.ban_queue.pop_front();

            match voter {
                Senate::R => {
                    self.radiant -= 1;
                }
                Senate::D => {
                    self.dire -= 1;
                }
            }
            // Banned voter gets no turn, continue election
            return Some(None);
        }

        if voter == Senate::R && self.dire == 0 {
            // Radiant victory!
            Some(Some("Radiant".to_owned()))
        } else if voter == Senate::D && self.radiant == 0 {
            // Dire victory!
            Some(Some("Dire".to_owned()))
        } else {
            // No victory yet, voter exercises their ban right
            // Voter goes to the back of the queue
            // Queues a ban for someone of the other side
            match voter {
                Senate::R => {
                    self.vote_queue.push_back(Senate::R);
                    self.ban_queue.push_back(Senate::D);
                }
                Senate::D => {
                    self.vote_queue.push_back(Senate::D);
                    self.ban_queue.push_back(Senate::R);
                }
            }
            // Continue election
            Some(None)
        }
    }
}

#[allow(dead_code)]
pub fn main() {
    let senate = String::from("DDRRR");
    assert_eq!(
        Solution::predict_party_victory(senate),
        String::from("Dire")
    );

    let senate = String::from("RD");
    assert_eq!(
        Solution::predict_party_victory(senate),
        String::from("Radiant")
    );
}
