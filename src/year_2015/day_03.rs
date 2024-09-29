use std::collections::HashSet;
use crate::Solution;
use crate::Solution::Solved;

const INPUT: &str = include_str!("day_03.txt");

#[derive(Default, Copy, Clone, Eq, PartialEq, Hash)]
struct Position(i32, i32);

impl Position {
    fn north(&mut self) {
        self.0 += 1
    }

    fn south(&mut self) {
        self.0 -= 1
    }

    fn east(&mut self) {
        self.1 += 1
    }

    fn west(&mut self) {
        self.1 -= 1
    }
}

pub fn part1() -> Solution {
    let mut position = Position::default();
    let mut set = HashSet::new();
    set.insert(position);
    for char in INPUT.chars() {
        match char {
            '^' => position.north(),
            '>' => position.east(),
            '<' => position.west(),
            'v' => position.south(),
            _ => {}
        }
        set.insert(position);
    }

    Solved(set.len() as i64)
}

pub fn part2() -> Solution {
    let mut position1 = Position::default();
    let mut position2 = Position::default();
    let mut set = HashSet::new();
    set.insert(position1);
    let mut is_robot_turn = false;
    for char in INPUT.chars() {
        let position = if is_robot_turn {
            &mut position1
        } else {
            &mut position2
        };
        is_robot_turn = !is_robot_turn;
        match char {
            '^' => position.north(),
            '>' => position.east(),
            '<' => position.west(),
            'v' => position.south(),
            _ => {}
        }
        set.insert(*position);
    }

    Solved(set.len() as i64)
}