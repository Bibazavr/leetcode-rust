// https://leetcode.com/problems/cheapest-flights-within-k-stops/

struct Solution;

impl Solution {
    pub fn find_cheapest_price(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
        let mut graph = vec![vec![]; n as usize];

        for flight in flights.iter() {
            let from = flight[0] as usize;
            let to = flight[1];
            let price = flight[2];

            graph[from].push((to, price));
        }

        let mut queue = std::collections::VecDeque::from([(src, 0, 0)]);

        let mut costs = vec![i32::MAX; n as usize];
        costs[src as usize] = 0;

        while let Some((from, cost, count_stops)) = queue.pop_front() {
            for &(dest, price) in &graph[from as usize] {
                let new_cost = cost + price;
                if new_cost < costs[dest as usize] && count_stops <= k {
                    queue.push_back((dest, new_cost, count_stops + 1));
                    costs[dest as usize] = new_cost;
                }
            }
        }

        match costs[dst as usize] {
            i32::MAX => -1,
            price => price,
        }
    }
}

#[allow(dead_code)]
pub fn main() {
    let n = 4;
    let flights = vec![
        vec![0, 1, 100],
        vec![1, 2, 100],
        vec![2, 0, 100],
        vec![1, 3, 600],
        vec![2, 3, 200],
    ];
    let src = 0;
    let dst = 3;
    let k = 1;

    assert_eq!(Solution::find_cheapest_price(n, flights, src, dst, k), 700);
}
