use crate::Solution;

const INPUT: &str = "1321131112";

pub fn part1() -> Solution {
    solve(40)
}

pub fn part2() -> Solution {
    solve(50)
}

fn solve(iterations: usize) -> Solution {
    let mut digits: Vec<_> = INPUT.chars()
        .map(|c| c.to_digit(10))
        .collect::<Option<_>>()
        .expect("failed to parse digits");
    for _ in 0..iterations {
        let mut count = 0;
        let mut last_digit = 0;
        digits = digits.into_iter().filter_map(|digit| {
            if count == 0 {
                // only possible at the star of the iterator
                last_digit = digit;
                count = 1;
                return None
            }
            if digit != last_digit {
                let result = [count, last_digit];
                count = 1;
                last_digit = digit;
                Some(result)
            } else {
                count += 1;
                None
            }
        }).flatten()
            .collect();
        digits.extend([count, last_digit])
    }
    Solution::Solved(digits.len() as i64)
}