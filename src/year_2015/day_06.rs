use std::str::FromStr;
use fixedbitset::FixedBitSet;
use crate::Solution;
use crate::Solution::Solved;

const INPUT: &str = include_str!("day_06.txt");

struct Position(u16, u16);

impl FromStr for Position {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut numbers = s.split(',').map(|n| n.parse().unwrap());
        Ok(Self(
            numbers.next().unwrap(),
            numbers.next().unwrap(),
        ))
    }
}

struct Range(Position, Position);

impl Range {
    fn iter(&self) -> impl Iterator<Item=usize> + '_ {
        (self.0.0..=self.1.0).flat_map(|x| {
            (self.0.1..=self.1.1).map(move |y| {
                (x as usize) * 1000 + y as usize
            })
        })
    }
}

struct Command {
    cmd: CommandType,
    range: Range,
}

impl FromStr for Command {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(' ');
        Ok(
            Command {
                cmd: match parts.next().unwrap() {
                    "toggle" => CommandType::Toggle,
                    "turn" => match parts.next().unwrap() {
                        "on" => CommandType::Turn(true),
                        "off" => CommandType::Turn(false),
                        _ => panic!()
                    },
                    _ => panic!()
                },
                range: Range(
                    parts.next().unwrap().parse().unwrap(),
                    {
                        parts.next().unwrap();
                        parts.next().unwrap().parse().unwrap()
                    },
                ),
            }
        )
    }
}

enum CommandType {
    Turn(bool),
    Toggle,
}

pub fn part1() -> Solution {
    let mut lights = FixedBitSet::with_capacity(1_000_000);
    for command in INPUT.lines().map(|line| Command::from_str(line).unwrap()) {
        match command.cmd {
            CommandType::Turn(state) => {
                for index in command.range.iter() {
                    lights.set(index, state)
                }
            }
            CommandType::Toggle => {
                for index in command.range.iter() {
                    lights.toggle(index)
                }
            }
        }
    }

    Solved(lights.count_ones(0..1_000_000) as i64)
}

pub fn part2() -> Solution {
    let mut lights = [0u8; 1_000_000];
    for command in INPUT.lines().map(|line| Command::from_str(line).unwrap()) {
        match command.cmd {
            CommandType::Turn(state) => {
                if state {
                    for index in command.range.iter() {
                        lights[index] += 1;
                    }
                } else {
                    for index in command.range.iter() {
                        if lights[index] > 0 {
                            lights[index] -= 1;
                        }
                    }
                }
            }
            CommandType::Toggle => {
                for index in command.range.iter() {
                    lights[index] += 2
                }
            }
        }
    }

    Solved(lights.into_iter().map(|u| u as i64).sum())
}