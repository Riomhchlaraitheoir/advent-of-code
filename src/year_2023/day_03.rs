use std::cmp::{max, Ordering};
use std::collections::BTreeMap;
use std::ops::{Range, Sub};
use itertools::Itertools;

use crate::Solution;

const INPUT: &str = include_str!("day_03.txt");

#[derive(Clone, Eq, PartialEq, Hash)]
struct Number {
    line: usize,
    columns: Range<usize>,
    number: u32,
}

impl Number {
    fn positions(&self) -> impl Iterator<Item=Position> + '_ {
        self.columns.clone().map(|col| Position {
            line: self.line,
            col,
        })
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
struct Position {
    line: usize,
    col: usize,
}

impl Position {
    fn neighbours(&self) -> impl Iterator<Item=Self> + '_ {
        [
            (-1_isize, -1_isize),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ].into_iter().filter_map(|(d_line, d_col)| {
            Some(Self {
                line: (self.line as isize).checked_add(d_line)? as usize,
                col: (self.col as isize).checked_add(d_col)? as usize,
            })
        })
    }
}

impl PartialOrd<Self> for Position {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(Ord::cmp(self, other))
    }
}

impl Ord for Position {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.line != other.line {
            self.line.cmp(&other.line)
        } else {
            self.col.cmp(&other.col)
        }
    }
}

impl Sub for Position {
    type Output = usize;

    fn sub(self, rhs: Self) -> Self::Output {
        max(self.line.abs_diff(rhs.line), self.col.abs_diff(rhs.col))
    }
}

pub fn part1() -> Solution {
    let symbols: Vec<_> = INPUT.lines().enumerate().flat_map(|(line_index, line)| {
        line.chars()
            .enumerate()
            .filter(|(_, c)| !c.is_ascii_digit() && *c != '.')
            .map(move |(i, _)| Position { line: line_index, col: i })
    }).collect();

    Solution::Solved(INPUT.lines().enumerate().flat_map(|(line_index, line)| {
        NumberIterator {
            chars: line.chars().enumerate(),
            line: line_index,
        }
    }).filter(|number| {
        symbols.iter().copied().any(|pos| number.positions().any(|digit| pos - digit <= 1))
    }).map(|number| number.number as i64)
        .sum())
}

pub fn part2() -> Solution {
    part2_impl(INPUT)
}

pub fn part2_impl(input: &str) -> Solution {
    let numbers: BTreeMap<_, _> = input.lines().enumerate().flat_map(|(line_index, line)| {
        NumberIterator {
            chars: line.chars().enumerate(),
            line: line_index,
        }
    }).flat_map(|number| {
        number.positions().map(|pos| (pos, number.clone())).collect_vec()
    }).collect();
    Solution::Solved(
        input.lines().enumerate().flat_map(|(line_index, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| {
                    *c == '*'
                })
                .map(move |(i, _)| Position { line: line_index, col: i })
        }).filter_map(|position| {
            let (a, b): (Number, Number) = position.neighbours().filter_map(|pos| {
                numbers.get(&pos).cloned()
            }).unique().collect_tuple()?;
            Some((a.number * b.number) as i64)
        }).sum()
    )
}

struct NumberIterator<Chars> {
    chars: Chars,
    line: usize,
}

impl<Chars: Iterator<Item=(usize, char)>> Iterator for NumberIterator<Chars> {
    type Item = Number;

    fn next(&mut self) -> Option<Self::Item> {
        let (mut i, mut c) = self.chars.next()?;
        while !c.is_ascii_digit() {
            (i, c) = self.chars.next()?;
        }
        let mut number = String::new();
        let mut length = 0;
        while c.is_ascii_digit() {
            number.push(c);
            length += 1;
            let Some((_, next)) = self.chars.next() else { break; };
            c = next
        }
        let number: u32 = number.parse().expect("failed to parse int");
        Some(Number {
            line: self.line,
            columns: i..(i + length),
            number,
        })
    }
}

#[cfg(test)]
mod test {
    use crate::Solution;

    #[test]
    fn part2() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        assert_eq!(Solution::Solved(467835), super::part2_impl(input))
    }
}