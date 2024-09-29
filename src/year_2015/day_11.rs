use crypto::common::generic_array::functional::FunctionalSequence;
use itertools::Itertools;
use crate::Solution;

const INPUT: &str = "vzbxkghb";
const VALID_CHARS: &str = "abcdefghijklmnopqrstuvwxyz";

pub fn part1() -> Solution {
    let password: [u8; 8] = INPUT.chars().map(|c| -> u8  {
        VALID_CHARS.chars().find_position(|o| c == *o).unwrap().0.try_into().unwrap()
    }).collect::<Vec<_>>().try_into().unwrap();

    let passwords = PasswordIterator {
        starting: password,
        current: password,
    };
    let result = passwords.filter(is_valid).next().unwrap();
    Solution::Solved(result)
}

fn is_valid(password: [u8;8]) -> bool {
    if !password.iter().copied().tuple_windows().any(|(a, b, c)| {
        b == a+1 && c == a + 2
    }) {
        return false
    }
    if password.contains(&8) || password.contains(&11) || password.contains(&14) {
        return false
    }
    let mut first_pair = None;
    for (a, b) in password.iter().copied().tuple_windows() {
        if a == b {
            let Some(first_pair) = first_pair else {
                first_pair = Some(a);
                continue
            };
            if first_pair == a {
                continue
            }
            return true
        }
    }
    false
}

pub fn part2() -> Solution {
    Solution::Unsolved
}

struct PasswordIterator {
    starting: [u8;8],
    current: [u8;8]
}

impl Iterator for PasswordIterator {
    type Item = [u8;8];

    fn next(&mut self) -> Option<Self::Item> {
        for i in (0..8).rev() {
            self.current[i] += 1;
            if self.current[i] == 23 {
                self.current[i] = 0;
            } else {
                break
            }
        }
        if self.current == self.starting {
            return None
        }

        Some(self.current)
    }
}
