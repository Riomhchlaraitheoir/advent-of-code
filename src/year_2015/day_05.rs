use std::collections::HashSet;
use std::ops::Index;

use crate::Solution;
use crate::Solution::Solved;

const VOWELS: &str = "aeoiu";
const NAUGHTY_WORDS: [&str; 4] = ["ab", "cd", "pq", "xy"];

const INPUT: &str = include_str!("day_05.txt");

pub fn part1() -> Solution {
    count_nice(|line|  {
        let vowels: HashSet<char> = VOWELS.chars().collect();
        if line.chars().filter(|c| vowels.contains(c)).count() < 3 {
            return false
        }
        {
            let mut has_pair = false;
            let mut chars = line.chars();
            let mut prev_char = chars.next().unwrap();
            for c in chars {
                if c == prev_char {
                    has_pair = true;
                    break;
                }
                prev_char = c;
            }
            if !has_pair {
                return false;
            }
        }
        let naughty_words: HashSet<_> = NAUGHTY_WORDS.into_iter().collect();
        for i in 0..line.len()-1 {
            if naughty_words.contains(line.index(i..(i+2))) {
                return false
            }
        }

        true
    })
}

pub fn part2() -> Solution {
    count_nice(|line| {
        {
            let mut pairs = HashSet::new();
            let mut chars = line.chars();
            let mut prev_char = chars.next().unwrap();
            let mut prev_pair = (0 as char, prev_char);
            let mut has_repeated_pair = false;
            for c in chars {
                let pair = (prev_char, c);
                if pairs.contains(&pair) && pair != prev_pair {
                    has_repeated_pair = true;
                    break;
                }
                pairs.insert(pair);
                prev_char = c;
                prev_pair = pair;
            }
            if !has_repeated_pair {
                return false;
            }
        }
        {
            let mut chars = line.chars();
            let mut prev_prev_char = chars.next().unwrap();
            let mut prev_char = chars.next().unwrap();
            let mut has_triple = false;
            for c in chars {
                if prev_prev_char == c {
                    has_triple = true;
                    break;
                }
                prev_prev_char = prev_char;
                prev_char = c;
            }
            if !has_triple {
                return false;
            }
        }
        true
    })
}

fn count_nice(is_nice: impl Fn(&str) -> bool) -> Solution {
    Solved(INPUT.lines().filter(|line| is_nice(line)).count() as i64)
}