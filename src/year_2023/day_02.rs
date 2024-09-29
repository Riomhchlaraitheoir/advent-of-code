use std::convert::Infallible;
use std::str::FromStr;
use crate::Solution;

const INPUT: &str = include_str!("day_02.txt");

#[derive(Debug)]
struct Game {
    id: u32,
    rounds: Vec<Round>,
}

#[derive(Debug, Default)]
struct Round {
    red: u32,
    green: u32,
    blue: u32,
}

impl FromStr for Game {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if &s[0..5] != "Game " {
            panic!("line should have prefix")
        }
        let colon = s.chars().enumerate().find(|(_, c)| *c == ':').expect("no colon found").0;
        let id: u32 = s[5..colon].parse().expect("cannot parse id");

        let rounds = s[(colon + 2)..].split(';').map(|round| {
            let mut red = 0_u32;
            let mut green = 0_u32;
            let mut blue = 0_u32;
            let round = round.trim();
            round.split(',').for_each(|colour| {
                let colour = colour.trim();
                let space = colour.chars().enumerate().find(|(_, c)| *c == ' ').expect("space not found").0;
                let count: u32 = colour[..space].parse().expect("cannot parse count");
                let colour = &colour[(space + 1)..];
                let counter = match colour {
                    "red" => &mut red,
                    "green" => &mut green,
                    "blue" => &mut blue,
                    _ => panic!("colour {colour} not recognised")
                };
                *counter += count
            });
            Round { red, green, blue }
        }).collect();

        Ok(Game {
            id,
            rounds,
        })
    }
}

pub fn part1() -> Solution {
    Solution::Solved(
        INPUT.lines().map(|line| Game::from_str(line).unwrap())
            .filter(|game| {
                game.rounds.iter().map(|round| round.red).max().unwrap() <= 12 &&
                    game.rounds.iter().map(|round| round.green).max().unwrap() <= 13 &&
                    game.rounds.iter().map(|round| round.blue).max().unwrap() <= 14
            }).map(|round| round.id).sum::<u32>() as i64
    )
}

pub fn part2() -> Solution {
    Solution::Solved(
        INPUT.lines().map(|line| Game::from_str(line).unwrap())
            .map(|game| {
                game.rounds.iter().map(|round| round.red).max().unwrap() *
                    game.rounds.iter().map(|round| round.green).max().unwrap() *
                    game.rounds.iter().map(|round| round.blue).max().unwrap()
            }).sum::<u32>() as i64
    )
}