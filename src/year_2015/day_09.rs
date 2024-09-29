use crate::Solution;
use itertools::Itertools;
use prse::parse;
use std::collections::HashMap;

pub fn possible_routes() -> impl Iterator<Item = u32> {
    let input = include_str!("day_09.txt");
    let edges: Vec<_> = input.lines().map(|line| {
        let from: &str;
        let to: &str;
        let distance: u32;
        parse!(line, "{from} to {to} = {distance}");
        (from, to, distance)
    }).collect();
    let nodes: Vec<_> = edges.iter()
        .copied()
        .flat_map(|(from, to, _)| [from, to])
        .unique()
        .collect();
    let costs: HashMap<_, _> = edges.iter().flat_map(|(from, to, distance)| {
        [((*from, *to), *distance), ((*to, *from), *distance)]
    }).collect();
    let node_count = nodes.len();
    nodes.into_iter()
        .permutations(node_count)
        .map(move |path| -> u32 {
            let distance = path.iter()
                .copied()
                .tuple_windows()
                .map(|leg| {
                    costs[&leg]
                })
                .sum();
            distance
        })
}

pub fn part1() -> Solution {
    Solution::Solved(possible_routes().min().unwrap() as i64)
}
pub fn part2() -> Solution {
    Solution::Solved(possible_routes().max().unwrap() as i64)
}