#![feature(step_trait)]
extern crate core;

use core::time;
use std::fmt::{Display, Formatter};
use std::time::Instant;
use clap::Parser;
use log::Level;

#[derive(Parser, Debug)]
#[command(author, about, version)]
struct Args {
    year: Option<u16>,
    day: Option<u8>
}

fn main() {
    let args = Args::parse();
    simple_logger::init_with_level(Level::Debug).unwrap();

    for (year, days) in YEARS {
        if let Some(filter) = args.year {
            if filter != year {
                continue
            }
        }
        println!("Year {year}:");
        for (day, solver) in days.iter().enumerate() {
            let day = day + 1;
            if let Some(filter) = args.day {
                if filter as usize != day {
                    continue
                }
            }
            println!("\tDay {day}:");


            let (result, duration) = run(solver.part1);
            if result == Solution::Unsolved {
                println!("\t\tPart 1: Not solved")
            } else {
                println!("\t\tPart 1: {result} ({duration:?})");
            }

            let (result, duration) = run(solver.part2);
            if result == Solution::Unsolved {
                println!("\t\tPart 2: Not solved")
            } else {
                println!("\t\tPart 2: {result} ({duration:?})");
            }
        }
    }
}

struct Solver {
    part1: fn() -> Solution,
    part2: fn() -> Solution
}

fn run(solver: fn() -> Solution) -> (Solution, time::Duration) {
    let start = Instant::now();
    let result = solver();
    let duration = Instant::now() - start;
    (result, duration)
}

#[derive(Eq, PartialEq, Clone, Debug)]
enum Solution {
    Solved(String),
    Unsolved
}

impl Display for Solution {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Solution::Solved(solution) => write!(f, "{}", solution),
            Solution::Unsolved => write!(f, "Unsolved")
        }
    }
}

const YEARS: [(u16, [Solver; 25]); 2] = [
    (2015, year_2015::DAYS),
    (2023, year_2023::DAYS)
];

#[macro_export]
macro_rules! solver {
    ($module: ident) => {
        Solver {
            part1: || $module::part1(),
            part2: || $module::part2()
        }
    };
}

#[macro_export]
macro_rules! year_module {
    ($name: ident) => {
    mod $name {
        mod day_01;
        mod day_02;
        mod day_03;
        mod day_04;
        mod day_05;
        mod day_06;
        mod day_07;
        mod day_08;
        mod day_09;
        mod day_10;
        mod day_11;
        mod day_12;
        mod day_13;
        mod day_14;
        mod day_15;
        mod day_16;
        mod day_17;
        mod day_18;
        mod day_19;
        mod day_20;
        mod day_21;
        mod day_22;
        mod day_23;
        mod day_24;
        mod day_25;

        use $crate::Solver;

        pub const DAYS: [Solver; 25] = [
            solver!(day_01),
            solver!(day_02),
            solver!(day_03),
            solver!(day_04),
            solver!(day_05),
            solver!(day_06),
            solver!(day_07),
            solver!(day_08),
            solver!(day_09),
            solver!(day_10),
            solver!(day_11),
            solver!(day_12),
            solver!(day_13),
            solver!(day_14),
            solver!(day_15),
            solver!(day_16),
            solver!(day_17),
            solver!(day_18),
            solver!(day_19),
            solver!(day_20),
            solver!(day_21),
            solver!(day_22),
            solver!(day_23),
            solver!(day_24),
            solver!(day_25),
        ];
    }
    };
}

year_module!(year_2015);
year_module!(year_2023);