// https://leetcode.com/problems/minimum-fuel-cost-to-report-to-the-capital/

struct Solution;

impl Solution {
    pub fn minimum_fuel_cost(roads: Vec<Vec<i32>>, seats: i32) -> i64 {
        let n = roads.len() + 1;
        let mut fuel = 0;
        let mut graph = vec![vec![]; n];

        for road in roads {
            graph[road[0] as usize].push(road[1] as usize);
            graph[road[1] as usize].push(road[0] as usize);
        }

        fuel += Self::dfs_travel(0, 0, &graph, seats as _).1;
        fuel
    }

    fn dfs_travel(i: usize, from: usize, graph: &Vec<Vec<usize>>, seats: i64) -> (i64, i64) {
        let mut node_size = 1;
        let mut fuel = 0;
        for &city in graph[i].iter() {
            if city != from {
                let (new_node_size, _spent_fuel) = Self::dfs_travel(city, i, graph, seats);
                fuel += _spent_fuel;
                node_size += new_node_size;
            }
        }

        if i != 0 {
            fuel += ((node_size - 1) / seats + 1) as i64;
        }

        (node_size, fuel)
    }
}

#[allow(dead_code)]
pub fn main() {
    let roads = vec![vec![0, 1], vec![0, 2], vec![1, 3], vec![1, 4]];
    let seats = 5;
    assert_eq!(Solution::minimum_fuel_cost(roads, seats), 4);

    let roads = vec![
        vec![3, 1],
        vec![3, 2],
        vec![1, 0],
        vec![0, 4],
        vec![0, 5],
        vec![4, 6],
    ];
    let seats = 2;
    assert_eq!(Solution::minimum_fuel_cost(roads, seats), 7);

    let roads = vec![vec![0, 1], vec![0, 2], vec![0, 3]];
    let seats = 5;
    assert_eq!(Solution::minimum_fuel_cost(roads, seats), 3);
}
