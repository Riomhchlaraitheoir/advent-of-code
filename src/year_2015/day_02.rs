use std::str::FromStr;
use crate::Solution;
use crate::Solution::Solved;

struct Box {
    l: u32,
    w: u32,
    h: u32,
}

impl FromStr for Box {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let x1 = s.find('x').expect("No x found");
        let x2 = s.rfind('x').expect("No second x found");
        Ok(
            Box {
                l: s[..x1].parse().expect("Failed to parse l"),
                w: s[(x1+1)..x2].parse().expect("Failed to parse l"),
                h: s[(x2+1)..].parse().expect("Failed to parse l"),
            }
        )
    }
}

const INPUT: &str = include_str!("day_02.txt");

pub fn part1() -> Solution {
    let total: u32 = INPUT.lines()
        .map(|line| Box::from_str(line).unwrap())
        .map(|bx| {
            let a = bx.l * bx.h;
            let b = bx.l * bx.w;
            let c = bx.h * bx.w;
            let area = (a+b+c) * 2;
            let extra = [a, b, c].into_iter().min().unwrap();
            area + extra
        }).sum();

    Solved(total as i64)
}

pub fn part2() -> Solution {
    let total: u32 = INPUT.lines()
        .map(|line| Box::from_str(line).unwrap())
        .map(|bx| {
            let volume = bx.l * bx.h * bx.w;
            let wrap = [bx.l + bx.h, bx.l + bx.w, bx.w + bx.h].into_iter()
                .min().unwrap() * 2;
            volume + wrap
        }).sum();

    Solved(total as i64)
}