use crate::Solution;
use crate::Solution::{Solved, Unsolved};

const INPUT: &str = "iwrupvqb";

pub fn part1() -> Solution {
    let key = INPUT.to_string();
    for i in 0.. {
        let input = key.clone() + &i.to_string();
        let output = md5::compute(input);
        if output[0] > 0 || output[1] > 0 || output[2] > 15 {
            continue
        }
        return Solved(i)
    }
    Unsolved
}

pub fn part2() -> Solution {
    let key = INPUT.to_string();
    for i in 0.. {
        let input = key.clone() + &i.to_string();
        let output = md5::compute(input);
        if output[0] > 0 || output[1] > 0 || output[2] > 0 {
            continue
        }
        return Solved(i)
    }
    Unsolved
}