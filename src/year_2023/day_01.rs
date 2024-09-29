use crate::Solution;

const INPUT: &str = include_str!("day_01.txt");

pub fn part1() -> Solution {
    Solution::Solved(INPUT.lines().map(|line| {
        if line.is_empty() {
            return 0;
        }
        let mut digits = line.chars().filter(char::is_ascii_digit);
        let first = digits.next().unwrap();
        let last = digits.last().unwrap_or(first);
        let first = first.to_digit(10).unwrap();
        let last = last.to_digit(10).unwrap();
        (first * 10 + last) as i64
    }).sum())
}

pub fn part2() -> Solution {
    Solution::Solved(INPUT.lines().map(|line| {
        if line.is_empty() {
            return 0;
        }
        let mut digits = NumberIterator {
            input: line.chars(),
            word_index: [0; 9],
        };
        let first = digits.next().unwrap();
        let last = digits.last().unwrap_or(first);
        (first * 10 + last) as i64
    }).sum())
}

struct NumberIterator<Input: Iterator<Item=char>> {
    input: Input,
    word_index: [usize; 9],
}

impl<Input: Iterator<Item=char>> Iterator for NumberIterator<Input> {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let c = self.input.next()?;
            if let Some(num) = c.to_digit(10) {
                return Some(num);
            }
            const WORDS: [&[char]; 9] = [
                &['o', 'n', 'e'],
                &['t', 'w', 'o'],
                &['t', 'h', 'r', 'e', 'e'],
                &['f', 'o', 'u', 'r'],
                &['f', 'i', 'v', 'e'],
                &['s', 'i', 'x'],
                &['s', 'e', 'v', 'e', 'n'],
                &['e', 'i', 'g', 'h', 't'],
                &['n', 'i', 'n', 'e'],
            ];
            for (i, word) in WORDS.into_iter().enumerate() {
                let count = self.word_index[i];
                self.word_index[i] = if c == word[count] {
                    count + 1
                } else if c == word[0] {
                    1
                } else {
                    0
                }
            }
            for (i, word) in WORDS.into_iter().enumerate() {
                if self.word_index[i] == word.len() {
                    self.word_index[i] = 0;
                    return Some((i + 1) as u32);
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::year_2023::day_01::NumberIterator;

    #[test]
    fn part2() {
        let lines = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
eighthree
sevenine";
        let numbers: Vec<Vec<_>> = lines.lines().map(|line| {
            let digits = NumberIterator {
                input: line.chars(),
                word_index: [0; 9],
            };
            digits.collect()
        }).collect();
        assert_eq!(numbers, vec![
            vec![2, 1, 9],
            vec![8, 2, 3],
            vec![1, 2, 3],
            vec![2, 1, 3, 4],
            vec![4, 9, 8, 7, 2],
            vec![1, 8, 2, 3, 4],
            vec![7, 6],
            vec![8,3],
            vec![7,9],
        ])
    }
}