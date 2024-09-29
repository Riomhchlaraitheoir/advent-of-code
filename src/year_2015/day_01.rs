use crate::Solution;
use crate::Solution::{Solved, Unsolved};

const INPUT: &str = include_str!("day_01.txt");

pub fn part1() -> Solution {
    let mut floor = 0;
    for char in INPUT.chars() {
        match char {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => {}
        }
    }

    Solution::Solved(floor)
}

pub fn part2() -> Solution {
    let mut floor = 0;
    for (index, char) in INPUT.chars().enumerate() {
        match char {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => {}
        }
        if floor < 0 {
            return Solved((index + 1) as i64)
        }
    }
    Unsolved
}