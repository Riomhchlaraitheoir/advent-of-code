#![allow(dead_code)]

use crate::Solution;
use prse::{parse, Parse};
use std::collections::HashMap;

#[derive(Debug, Parse, Clone)]
enum Expression<'a> {
    #[prse = "NOT {}"]
    Not(&'a str),
    #[prse = "{} AND {}"]
    And(&'a str, &'a str),
    #[prse = "{} OR {}"]
    Or(&'a str, &'a str),
    #[prse = "{} RSHIFT {}"]
    RShift(&'a str, u8),
    #[prse = "{} LSHIFT {}"]
    LShift(&'a str, u8),
    #[prse = "{}"]
    ConstantOrWire(&'a str),
}

#[derive(Debug, Default)]
struct Wires<'a> {
    unresolved: HashMap<String, Expression<'a>>,
    resolved: HashMap<String, u16>
}

impl<'a> FromIterator<&'a str> for Wires<'a> {
    fn from_iter<T: IntoIterator<Item=&'a str>>(lines: T) -> Self {
        let mut wires = Self::default();
        for line in lines {
            let expression: Expression;
            let target: String;
            parse!(line, "{expression} -> {target}");
            wires.unresolved.insert(target, expression);
        }
        wires
    }
}

impl Wires<'_> {
    fn resolve(&mut self, wire: &str) -> u16 {
        if let Ok(constant) = u16::from_str(wire) {
            return constant
        }
        if let Some(resolved) = self.resolved.get(wire).copied() {
            resolved
        } else {
            let expr = self.unresolved.get(wire).cloned().unwrap_or_else(|| panic!("wire {wire} not found"));
            let value = match expr {
                Expression::Not(wire) => !self.resolve(wire),
                Expression::And(left, right) => self.resolve(left) & self.resolve(right),
                Expression::Or(left, right) => self.resolve(left) | self.resolve(right),
                Expression::RShift(wire, n) => self.resolve(wire) >> n,
                Expression::LShift(wire, n) => self.resolve(wire) << n,
                Expression::ConstantOrWire(wire) => self.resolve(wire),
            };
            self.resolved.insert(wire.to_string(), value);
            value
        }
    }
}

const INPUT: &str = include_str!("day_07.txt");

pub fn part1() -> Solution {
    let mut wires: Wires = INPUT.lines().collect();

    let value = wires.resolve("a");
    Solution::Solved(value as i64)
}

pub fn part2() -> Solution {
    let mut wires: Wires = INPUT.lines().collect();
    let Solution::Solved(a) = part1() else { panic!("failed to get a"); };
    wires.resolved.insert("b".to_string(), a as u16);

    let value = wires.resolve("a");
    Solution::Solved(value as i64)
}