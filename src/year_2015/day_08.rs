use crate::Solution;

const INPUT: &str = include_str!("day_08.txt");

enum ParseState {
    Normal,
    Escaped,
    HexFirst,
    HexSecond,
}

pub fn part1() -> Solution {
    let mut codepoints = 0;
    let mut characters = 0;
    for line in INPUT.lines() {
        let line = line.strip_prefix('"').unwrap();
        let line = line.strip_suffix('"').unwrap();
        let mut state = ParseState::Normal;
        codepoints += 2;
        for c in line.chars() {
            codepoints += 1;
            match state {
                ParseState::Normal => {
                    if c == '\\' {
                        state = ParseState::Escaped;
                        continue
                    }
                    characters += 1;
                }
                ParseState::Escaped => {
                    match c {
                        '\\' | '\"' => {
                            state = ParseState::Normal;
                            characters += 1;
                            continue
                        }
                        'x' => {
                            state = ParseState::HexFirst;
                        }
                        _ => {
                            panic!("Unknown escaped character: {c}")
                        }
                    }
                }
                ParseState::HexFirst => {
                    state = ParseState::HexSecond
                }
                ParseState::HexSecond => {
                    state = ParseState::Normal;
                    characters += 1;
                }
            }

        }
    }

    Solution::Solved(codepoints - characters)
}

pub fn part2() -> Solution {
    let mut codepoints = 0;
    let mut encoded = 0;
    for line in INPUT.lines() {
        let line = line.strip_prefix('"').unwrap();
        let line = line.strip_suffix('"').unwrap();
        codepoints += 2;
        encoded += 6;
        for c in line.chars() {
            codepoints += 1;
            match c {
                '\\' | '\"' => {
                    encoded += 2;
                }
                _ => {
                    encoded += 1;
                }
            }
        }
    }

    Solution::Solved(encoded - codepoints)
}
